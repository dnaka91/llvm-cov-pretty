# Syntect Themes

This is a collection of Git submodules, taken from the [bat](https://github.com/sharkdp/bat/tree/master/assets/themes) project. It extends the relatively small default theme list of [syntect](https://docs.rs/syntect/latest/syntect/highlighting/struct.ThemeSet.html#method.load_defaults).

These are all pre-processed during build-time. Specifically each theme is turned into CSS and minified, then directly compiled into the binary. This makes it a quick operation during runtime, as now the processed string can be copied over into a file without any further steps.
