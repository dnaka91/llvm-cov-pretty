# Changelog

All notable changes to this project will be documented in this file.

<!-- markdownlint-disable no-duplicate-header -->
<!-- markdownlint-disable no-trailing-spaces -->

## [0.1.6](https://github.com/dnaka91/wazzup/compare/v0.1.5...v0.1.6) - 2023-08-23

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Unpin serde again ([2ee8806](https://github.com/dnaka91/wazzup/commit/2ee8806a488f6a6d405772f16dd236f7834fb6d9))
  > The issue with pre-compiled binaries in `serde` has been resolved.
  > Therefore, the pin can be removed, but a minimum version of `1.0.185` is
  > required.

## [0.1.5](https://github.com/dnaka91/wazzup/compare/v0.1.4...v0.1.5) - 2023-08-19

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Avoid pre-compiled binaries from serde ([6ea517d](https://github.com/dnaka91/wazzup/commit/6ea517dda075136ff85b8b26b3c5c418068abf32))
  > Pin `serde` to `1.0.171` to avoid pre-compiled binaries that are used
  > after for derive macros.

## [0.1.4](https://github.com/dnaka91/wazzup/compare/v0.1.3...v0.1.4) - 2023-08-10

### <!-- 0 -->⛰️ Features

- Report the JSON path of in case of a parsing error ([b0a3497](https://github.com/dnaka91/wazzup/commit/b0a349721e48cb60f04a40484e4601d5a518e721))
  > If deserialization of the coverage data failed, and the cause is an
  > invalid or unexpected structure, the JSON path to the location of the
  > error is reported.
- Use new cargo-llvm-cov context data ([a03b5cf](https://github.com/dnaka91/wazzup/commit/a03b5cf89fc1f8479b93d652f20da9e41cdeb2b2))
  > With a recent release of `cargo-llvm-cov`, additional data is injected
  > into the coverage JSON report. This allows for easier version checks and
  > location of the project's manifest file.

### <!-- 4 -->🚜 Refactor

- Don't double-print the JSON path ([9f84893](https://github.com/dnaka91/wazzup/commit/9f84893028b7750134051d76b47331fc8ed9f766))

### <!-- 6 -->🧪 Testing

- Add unit tests for highlighter, minifier and schema parsing ([6f0d782](https://github.com/dnaka91/wazzup/commit/6f0d782c065a2c6b4a6e7c6bd7b99cbadabc0a0f))

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Update dependencies ([dd1892f](https://github.com/dnaka91/wazzup/commit/dd1892f5cbcdfd48344036236441339e52c23d68))
- Switch form yarn to pnpm ([cb80299](https://github.com/dnaka91/wazzup/commit/cb8029936a5257f7ef5633aab96a306f66e23ad6))

## [0.1.3](https://github.com/dnaka91/wazzup/compare/v0.1.2...v0.1.3) - 2023-06-23

### <!-- 0 -->⛰️ Features

- Add option to automatically open the report and print the location otherwise ([88fb877](https://github.com/dnaka91/wazzup/commit/88fb87797fd39722e13c2dd05d7f40b45c93b615))
- Use the relative path in the source page to be in sync with the index ([67c9816](https://github.com/dnaka91/wazzup/commit/67c98161a5dd358511ecce1b9c6cedd71025da48))

### <!-- 1 -->🐛 Bug Fixes

- Cover the full line range of instantiation data ([9303b6b](https://github.com/dnaka91/wazzup/commit/9303b6b827d40e643ed5e00ec04afef0324f3d4f))
  > At first, it looked like the instantiation coverage always defines a
  > single line, but as it turns out there are cases where it streches over
  > more than one line.
  > 
  > This data must be included to properly display the overrides for
  > uncovered instantiations.

## [0.1.2](https://github.com/dnaka91/wazzup/compare/v0.1.1...v0.1.2) - 2023-06-22

### <!-- 0 -->⛰️ Features

- Improve error messages, especially external tool checks ([3788323](https://github.com/dnaka91/wazzup/commit/378832381d58ef6a43e755c90a94d34a26f6d2fb))
- Allow manually defining the project's manifest path ([5d972b8](https://github.com/dnaka91/wazzup/commit/5d972b8e00e5d337d30abfa0a3e875723b593bf2))
- Make annotations for missing instantations opt-in ([d798f5a](https://github.com/dnaka91/wazzup/commit/d798f5ae7196174680d3187eb6d16ae17529a0db))
- Ensure files lists are always sorted alphabetically ([2b898f8](https://github.com/dnaka91/wazzup/commit/2b898f8181b5ce5d318e2f3e3625a93d53aa4ea8))
- Add option to only highlight the gutter for coverage info ([97b359c](https://github.com/dnaka91/wazzup/commit/97b359c08a46a96e33cab6197ba706c865987822))

### <!-- 1 -->🐛 Bug Fixes

- Ensure partially covered lines are always marked as uncovered ([f4b3f0f](https://github.com/dnaka91/wazzup/commit/f4b3f0fbe0b1782c346e230d23cab21d97b6abab))

### <!-- 2 -->📚 Documentation

- Note about first setup and stability ([9bcc4f3](https://github.com/dnaka91/wazzup/commit/9bcc4f3b1a9fc485115260057a3ea1163c2342dc))
- Show light/dark theme support in help messages ([5fa713f](https://github.com/dnaka91/wazzup/commit/5fa713ff627e5f677eb9ba758e2b5b6247f988ce))

### <!-- 4 -->🚜 Refactor

- Adjust schema structure to only accept a single file name ([1443ab1](https://github.com/dnaka91/wazzup/commit/1443ab1d12f877f465f675246327c71de898a155))
- Improve CLI argument configuration ([e7ca3cb](https://github.com/dnaka91/wazzup/commit/e7ca3cb48618ea3e9de02a66be725a4dff084a52))

## [0.1.1](https://github.com/dnaka91/wazzup/compare/v0.1.0...v0.1.1) - 2023-06-22

### <!-- 0 -->⛰️ Features

- Process coverage data and files in parallel ([d253d0f](https://github.com/dnaka91/wazzup/commit/d253d0f562cc585700f46d53bbccb9c1e2703ea9))
- Add version check for cargo-llvm-cov ([624ab1c](https://github.com/dnaka91/wazzup/commit/624ab1c6cf4f18df31a33ca48e4c545f1e78e6c2))

### <!-- 1 -->🐛 Bug Fixes

- Don't show a 0 count if coverage is unknown ([6cd93f8](https://github.com/dnaka91/wazzup/commit/6cd93f8cd7788cda76d874c365277433ee5e117c))
- Reduce gaps in colored coverage areas ([1094cfa](https://github.com/dnaka91/wazzup/commit/1094cfad0909381a3c118a128b2b2d387b02898f))
- Handle the case where report paths are relative ([94ae77a](https://github.com/dnaka91/wazzup/commit/94ae77a6b8ac213f22b40cb43382170458ff9970))

### <!-- 2 -->📚 Documentation

- Partially document code parts ([2518ff7](https://github.com/dnaka91/wazzup/commit/2518ff74e27caf081cb8114eaa277eef78c80b2a))
- Include commit hashes in the changelog ([d5befa1](https://github.com/dnaka91/wazzup/commit/d5befa1a7aa7dd93f09ec2a5e15d9d1f25fc5a93))

### <!-- 4 -->🚜 Refactor

- Use askama's escape function during highlighting ([e599493](https://github.com/dnaka91/wazzup/commit/e599493e6c5f7003a262ed6fb8b913de9c92caca))
- Simplify function name demangling ([52ff0ea](https://github.com/dnaka91/wazzup/commit/52ff0ea739cbaeb2ecb766611540027c3171ba68))
- Improve logic to locate the project root ([593d315](https://github.com/dnaka91/wazzup/commit/593d315c13a5988f05e9cb1d190fd9b10534a718))

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Configure GitHub issue templates ([5eaa073](https://github.com/dnaka91/wazzup/commit/5eaa073e87937e0361cd5818844fbfc06ee5ca34))
- Don't render all issue template fields as Markdown ([93de0d1](https://github.com/dnaka91/wazzup/commit/93de0d109aedc3de9fae2f004a4b9efd3156fd63))

## [0.1.0](https://github.com/dnaka91/wazzup/releases/tag/v0.1.0) - 2023-06-18

### <!-- 7 -->⚙️ Miscellaneous Tasks

- Initial commit ([d8b1afe](https://github.com/dnaka91/wazzup/commit/d8b1afe2b1efa06b3b78d7fa7789b963b1df8dd2))

<!-- generated by git-cliff -->
