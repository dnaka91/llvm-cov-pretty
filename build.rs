use std::{
    fs::{self, File},
    io::BufReader,
};

use heck::{ToKebabCase, ToPascalCase};
use quote::{format_ident, quote};
use syntect::{
    highlighting::ThemeSet,
    html::{self, ClassStyle},
};

const BUILTIN: &[(&str, &str, &str)] = &[
    ("solarized", "Solarized (light)", "Solarized (dark)"),
    ("base16_ocean", "base16-ocean.light", "base16-ocean.dark"),
];

const CUSTOM: &[(&str, Option<&str>, Option<&str>)] = &[
    (
        "one_three_three_seven",
        None,
        Some("1337-Scheme/1337.tmTheme"),
    ),
    (
        "coldark",
        Some("Coldark/Coldark-Cold.tmTheme"),
        Some("Coldark/Coldark-Dark.tmTheme"),
    ),
    ("dark_neon", None, Some("DarkNeon/DarkNeon.tmTheme")),
    ("dracula", None, Some("dracula-sublime/Dracula.tmTheme")),
    ("github", Some("github-sublime-theme/GitHub.tmTheme"), None),
    (
        "gruvbox",
        Some("gruvbox/gruvbox-light.tmTheme"),
        Some("gruvbox/gruvbox-dark.tmTheme"),
    ),
    ("nord", None, Some("Nord-sublime/Nord.tmTheme")),
    (
        "one_half",
        Some("onehalf/sublimetext/OneHalfLight.tmTheme"),
        Some("onehalf/sublimetext/OneHalfDark.tmTheme"),
    ),
    // Solarized
    (
        "monokai_extended",
        Some("sublime-monokai-extended/Monokai Extended Light.tmTheme"),
        Some("sublime-monokai-extended/Monokai Extended.tmTheme"),
    ),
    (
        "snazzy",
        None,
        Some("sublime-snazzy/Sublime Snazzy.tmTheme"),
    ),
    ("two_dark", None, Some("TwoDark/TwoDark.tmTheme")),
    (
        "visual_studio_plus",
        None,
        Some("visual-studio-dark-plus/Visual Studio Dark+.tmTheme"),
    ),
    ("zenburn", None, Some("zenburn/zenburn.tmTheme")),
];

fn main() {
    let ts = ThemeSet::load_defaults();
    let style = ClassStyle::SpacedPrefixed { prefix: "syntect-" };

    let out = std::env::var("OUT_DIR").unwrap();

    for &(name, light, dark) in BUILTIN {
        let light = html::css_for_theme_with_class_style(&ts.themes[light], style).unwrap();
        let dark = html::css_for_theme_with_class_style(&ts.themes[dark], style).unwrap();

        fs::write(
            format!("{out}/{name}.css"),
            minify(format!(
                "{light}\n@media (prefers-color-scheme:dark) {{\n{dark}}}"
            )),
        )
        .unwrap();
    }

    for &(name, light, dark) in CUSTOM {
        let light = light.map(|light| {
            println!("cargo:rerun-if-changed=assets/themes/{light}");
            html::css_for_theme_with_class_style(
                &ThemeSet::load_from_reader(&mut BufReader::new(
                    File::open(format!("assets/themes/{light}")).unwrap(),
                ))
                .unwrap(),
                style,
            )
            .unwrap()
        });
        let dark = dark.map(|dark| {
            println!("cargo:rerun-if-changed=assets/themes/{dark}");
            html::css_for_theme_with_class_style(
                &ThemeSet::load_from_reader(&mut BufReader::new(
                    File::open(format!("assets/themes/{dark}")).unwrap(),
                ))
                .unwrap(),
                style,
            )
            .unwrap()
        });

        let theme = match (light, dark) {
            (Some(theme), None) | (None, Some(theme)) => theme,
            (Some(light), Some(dark)) => {
                format!("{light}\n@media (prefers-color-scheme:dark) {{\n{dark}}}")
            }
            (None, None) => panic!("neither light nor dark theme"),
        };

        fs::write(format!("{out}/{name}.css"), minify(theme)).unwrap();
    }

    let names = BUILTIN
        .iter()
        .map(|v| v.0)
        .chain(CUSTOM.iter().map(|v| v.0));

    let variants = names.clone().map(|name| {
        let variant = format_ident!("{}", name.to_pascal_case());
        quote! { #variant }
    });
    let contents = names.clone().map(|name| {
        let variant = format_ident!("{}", name.to_pascal_case());
        let path = format!("{out}/{name}.css");
        quote! { Self::#variant => include_str!(#path) }
    });
    let displays = names.map(|name| {
        let variant = format_ident!("{}", name.to_pascal_case());
        let display = name.to_kebab_case();
        quote! { Self::#variant => #display }
    });

    let tokens = quote! {
        /// Possible themes that can be used for code highlighting.
        #[derive(Clone, Copy, ::clap::ValueEnum)]
        pub enum Theme {
            #(#variants,)*
        }

        impl Theme {
            pub fn as_str(self) -> &'static str {
                match self {
                    #(#contents,)*
                }
            }

            pub fn as_bytes(self) -> &'static [u8] {
                self.as_str().as_bytes()
            }
        }

        impl std::fmt::Display for Theme {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    #(#displays,)*
                })
            }
        }
    };

    fs::write(format!("{out}/styles.rs"), tokens.to_string()).unwrap();
}

fn minify(css: impl AsRef<str>) -> String {
    css_minify::optimizations::Minifier::default()
        .minify(css.as_ref(), css_minify::optimizations::Level::One)
        .unwrap()
}
