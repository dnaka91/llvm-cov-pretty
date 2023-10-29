# Changelog

All notable changes to this project will be documented in this file.

<!-- markdownlint-disable no-duplicate-header -->
<!-- markdownlint-disable no-trailing-spaces -->
## Unreleased

### ⚙️ Miscellaneous Tasks

- Bump actions/setup-node to v4 ([83fed52](https://github.com/dnaka91/llvm-cov-pretty/commit/83fed5275c8cedb9743a66a40463c03275dd150a))

## [0.1.9](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.8...v0.1.9) - 2023-10-10

### 🐛 Bug Fixes

- Remove non-existing extra schema fields ([d9e67b7](https://github.com/dnaka91/llvm-cov-pretty/commit/d9e67b762936b62d132ac5bbee9fee7b01d9fa6d))
  > These extra fields that `cargo-llvm-cov` adds were originally considered
  > but never actually made it in the schema. Thus, they cause the schema
  > parsing to fail.

## [0.1.8](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.7...v0.1.8) - 2023-10-06

### ⛰️ Features

- Allow to control the report's output directory ([458ede9](https://github.com/dnaka91/llvm-cov-pretty/commit/458ede98a235693ef7af80d913a744587b9e29d3))
  > A new CLI flag that allows to save the report to a user-defined
  > directory instead of the default `target/llvm-cov-pretty` location.
  > 
  > Caution should be taken as there is no additional logic for non-empty
  > directories. The target directory will be fully wiped in the process of
  > generating the report, without prior confirmation.
- Enable auto-wrapping of CLI help messages ([86cd5c7](https://github.com/dnaka91/llvm-cov-pretty/commit/86cd5c7ad853570095e5aa0aec2fff39ce9b6c7e))
  > By enabling clap's `wrap_help` feature, the help output is now nicely
  > aligned to the terminal wraps, which makes it easier to read in case of
  > longer help messages.

### 🐛 Bug Fixes

- Set link for first uncovered line ([481434a](https://github.com/dnaka91/llvm-cov-pretty/commit/481434a271553ce5fea32bb1efe442a46611776b))
  > A link tag existed that should move to the first uncovered line in
  > source view, but it never contained a proper working URL.
  > 
  > Also, the link won't be generated anymore if there aren't any uncovered
  > lines in the file.

### 📚 Documentation

- Correct the repo URL of links in CHANGELOG.md ([178b848](https://github.com/dnaka91/llvm-cov-pretty/commit/178b84814997666d52fefbc533f902171000173a))
  > In the previous improvement of the changelog format, the wrong
  > repository URL slipped in the settings as the file was copied over from
  > another project. This resulted in all link to point to the wrong
  > location.

## [0.1.7](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.6...v0.1.7) - 2023-09-25

### ⛰️ Features

- Mark partially-covered lines in yellow ([e3eafbd](https://github.com/dnaka91/llvm-cov-pretty/commit/e3eafbdd3a9bd998a5d371a0369a19f998bc964d))
  > So far lines that are covered but have missing function calls
  > (instantiations) were marked with red. To better indicate that they're
  > basically called but have a few missing instantiations a new yellow
  > color is introduced for those.
- Sort and deduplicate annotation messages ([fbcaf0e](https://github.com/dnaka91/llvm-cov-pretty/commit/fbcaf0eb85681e555292fc1052e50c42a79947e2))
  > The opt-in annotations for missing function calls and instantiations
  > are now sorted alphabetically and deduplicated to reduce the amount of
  > vertical space occupied.
- Add a new flag to disable usage of function coverage ([5dd1824](https://github.com/dnaka91/llvm-cov-pretty/commit/5dd18245a7632b10eee7243af031fb79ff4c2791))
  > A new flag that allows to opt-out of utilizing the function coverage
  > information in the source view. That means only basic file coverage will
  > be used if this flag is used.
  > 
  > Overview pages will still show the function coverage percentages
  > accordingly, regardless of the flag.

### 📚 Documentation

- Improve changelog format ([2662278](https://github.com/dnaka91/llvm-cov-pretty/commit/266227895382700d0c56c434d4c75597094c2012))
  > Extend the changelog generator to create prettier changelogs, similar to
  > how the changelog in the `git-cliff` tool itself looks.

## [0.1.6](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.5...v0.1.6) - 2023-08-23

### ⚙️ Miscellaneous Tasks

- Unpin serde again ([2ee8806](https://github.com/dnaka91/llvm-cov-pretty/commit/2ee8806a488f6a6d405772f16dd236f7834fb6d9))
  > The issue with pre-compiled binaries in `serde` has been resolved.
  > Therefore, the pin can be removed, but a minimum version of `1.0.185` is
  > required.

## [0.1.5](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.4...v0.1.5) - 2023-08-19

### ⚙️ Miscellaneous Tasks

- Avoid pre-compiled binaries from serde ([6ea517d](https://github.com/dnaka91/llvm-cov-pretty/commit/6ea517dda075136ff85b8b26b3c5c418068abf32))
  > Pin `serde` to `1.0.171` to avoid pre-compiled binaries that are used
  > after for derive macros.

## [0.1.4](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.3...v0.1.4) - 2023-08-10

### ⛰️ Features

- Report the JSON path of in case of a parsing error ([b0a3497](https://github.com/dnaka91/llvm-cov-pretty/commit/b0a349721e48cb60f04a40484e4601d5a518e721))
  > If deserialization of the coverage data failed, and the cause is an
  > invalid or unexpected structure, the JSON path to the location of the
  > error is reported.
- Use new cargo-llvm-cov context data ([a03b5cf](https://github.com/dnaka91/llvm-cov-pretty/commit/a03b5cf89fc1f8479b93d652f20da9e41cdeb2b2))
  > With a recent release of `cargo-llvm-cov`, additional data is injected
  > into the coverage JSON report. This allows for easier version checks and
  > location of the project's manifest file.

### 🚜 Refactor

- Don't double-print the JSON path ([9f84893](https://github.com/dnaka91/llvm-cov-pretty/commit/9f84893028b7750134051d76b47331fc8ed9f766))

### 🧪 Testing

- Add unit tests for highlighter, minifier and schema parsing ([6f0d782](https://github.com/dnaka91/llvm-cov-pretty/commit/6f0d782c065a2c6b4a6e7c6bd7b99cbadabc0a0f))

### ⚙️ Miscellaneous Tasks

- Update dependencies ([dd1892f](https://github.com/dnaka91/llvm-cov-pretty/commit/dd1892f5cbcdfd48344036236441339e52c23d68))
- Switch form yarn to pnpm ([cb80299](https://github.com/dnaka91/llvm-cov-pretty/commit/cb8029936a5257f7ef5633aab96a306f66e23ad6))

## [0.1.3](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.2...v0.1.3) - 2023-06-23

### ⛰️ Features

- Add option to automatically open the report and print the location otherwise ([88fb877](https://github.com/dnaka91/llvm-cov-pretty/commit/88fb87797fd39722e13c2dd05d7f40b45c93b615))
- Use the relative path in the source page to be in sync with the index ([67c9816](https://github.com/dnaka91/llvm-cov-pretty/commit/67c98161a5dd358511ecce1b9c6cedd71025da48))

### 🐛 Bug Fixes

- Cover the full line range of instantiation data ([9303b6b](https://github.com/dnaka91/llvm-cov-pretty/commit/9303b6b827d40e643ed5e00ec04afef0324f3d4f))
  > At first, it looked like the instantiation coverage always defines a
  > single line, but as it turns out there are cases where it streches over
  > more than one line.
  > 
  > This data must be included to properly display the overrides for
  > uncovered instantiations.

## [0.1.2](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.1...v0.1.2) - 2023-06-22

### ⛰️ Features

- Improve error messages, especially external tool checks ([3788323](https://github.com/dnaka91/llvm-cov-pretty/commit/378832381d58ef6a43e755c90a94d34a26f6d2fb))
- Allow manually defining the project's manifest path ([5d972b8](https://github.com/dnaka91/llvm-cov-pretty/commit/5d972b8e00e5d337d30abfa0a3e875723b593bf2))
- Make annotations for missing instantations opt-in ([d798f5a](https://github.com/dnaka91/llvm-cov-pretty/commit/d798f5ae7196174680d3187eb6d16ae17529a0db))
- Ensure files lists are always sorted alphabetically ([2b898f8](https://github.com/dnaka91/llvm-cov-pretty/commit/2b898f8181b5ce5d318e2f3e3625a93d53aa4ea8))
- Add option to only highlight the gutter for coverage info ([97b359c](https://github.com/dnaka91/llvm-cov-pretty/commit/97b359c08a46a96e33cab6197ba706c865987822))

### 🐛 Bug Fixes

- Ensure partially covered lines are always marked as uncovered ([f4b3f0f](https://github.com/dnaka91/llvm-cov-pretty/commit/f4b3f0fbe0b1782c346e230d23cab21d97b6abab))

### 📚 Documentation

- Note about first setup and stability ([9bcc4f3](https://github.com/dnaka91/llvm-cov-pretty/commit/9bcc4f3b1a9fc485115260057a3ea1163c2342dc))
- Show light/dark theme support in help messages ([5fa713f](https://github.com/dnaka91/llvm-cov-pretty/commit/5fa713ff627e5f677eb9ba758e2b5b6247f988ce))

### 🚜 Refactor

- Adjust schema structure to only accept a single file name ([1443ab1](https://github.com/dnaka91/llvm-cov-pretty/commit/1443ab1d12f877f465f675246327c71de898a155))
- Improve CLI argument configuration ([e7ca3cb](https://github.com/dnaka91/llvm-cov-pretty/commit/e7ca3cb48618ea3e9de02a66be725a4dff084a52))

## [0.1.1](https://github.com/dnaka91/llvm-cov-pretty/compare/v0.1.0...v0.1.1) - 2023-06-22

### ⛰️ Features

- Process coverage data and files in parallel ([d253d0f](https://github.com/dnaka91/llvm-cov-pretty/commit/d253d0f562cc585700f46d53bbccb9c1e2703ea9))
- Add version check for cargo-llvm-cov ([624ab1c](https://github.com/dnaka91/llvm-cov-pretty/commit/624ab1c6cf4f18df31a33ca48e4c545f1e78e6c2))

### 🐛 Bug Fixes

- Don't show a 0 count if coverage is unknown ([6cd93f8](https://github.com/dnaka91/llvm-cov-pretty/commit/6cd93f8cd7788cda76d874c365277433ee5e117c))
- Reduce gaps in colored coverage areas ([1094cfa](https://github.com/dnaka91/llvm-cov-pretty/commit/1094cfad0909381a3c118a128b2b2d387b02898f))
- Handle the case where report paths are relative ([94ae77a](https://github.com/dnaka91/llvm-cov-pretty/commit/94ae77a6b8ac213f22b40cb43382170458ff9970))

### 📚 Documentation

- Partially document code parts ([2518ff7](https://github.com/dnaka91/llvm-cov-pretty/commit/2518ff74e27caf081cb8114eaa277eef78c80b2a))
- Include commit hashes in the changelog ([d5befa1](https://github.com/dnaka91/llvm-cov-pretty/commit/d5befa1a7aa7dd93f09ec2a5e15d9d1f25fc5a93))

### 🚜 Refactor

- Use askama's escape function during highlighting ([e599493](https://github.com/dnaka91/llvm-cov-pretty/commit/e599493e6c5f7003a262ed6fb8b913de9c92caca))
- Simplify function name demangling ([52ff0ea](https://github.com/dnaka91/llvm-cov-pretty/commit/52ff0ea739cbaeb2ecb766611540027c3171ba68))
- Improve logic to locate the project root ([593d315](https://github.com/dnaka91/llvm-cov-pretty/commit/593d315c13a5988f05e9cb1d190fd9b10534a718))

### ⚙️ Miscellaneous Tasks

- Configure GitHub issue templates ([5eaa073](https://github.com/dnaka91/llvm-cov-pretty/commit/5eaa073e87937e0361cd5818844fbfc06ee5ca34))
- Don't render all issue template fields as Markdown ([93de0d1](https://github.com/dnaka91/llvm-cov-pretty/commit/93de0d109aedc3de9fae2f004a4b9efd3156fd63))

## [0.1.0](https://github.com/dnaka91/llvm-cov-pretty/releases/tag/v0.1.0) - 2023-06-18

### ⚙️ Miscellaneous Tasks

- Initial commit ([d8b1afe](https://github.com/dnaka91/llvm-cov-pretty/commit/d8b1afe2b1efa06b3b78d7fa7789b963b1df8dd2))

<!-- generated by git-cliff -->
