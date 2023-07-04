//! Logic for minifying HTML content.

pub struct Minifier(minify_html::Cfg);

impl Minifier {
    pub fn new() -> Self {
        Self(minify_html::Cfg {
            keep_closing_tags: true,
            minify_css: true,
            minify_js: true,
            ..minify_html::Cfg::spec_compliant()
        })
    }

    pub fn minify(&self, html: impl AsRef<[u8]>) -> Vec<u8> {
        minify_html::minify(html.as_ref(), &self.0)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn run_minifier() {
        let output = super::Minifier::new().minify(indoc! {"
            <p>
                <span>te</span>
                <span>st</span>
            </p>
        "});
        let expect = "<p><span>te</span> <span>st</span></p>";

        assert_eq!(expect, String::from_utf8(output).unwrap());
    }
}
