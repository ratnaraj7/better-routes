# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/ratnaraj7/better-routes/compare/better-routes-macros-v0.3.0...better-routes-macros-v0.3.1) - 2024-10-03

### Other

- remove unnecessary `quote` macro

## [0.3.0](https://github.com/ratnaraj7/better-routes/compare/better-routes-macros-v0.2.0...better-routes-macros-v0.3.0) - 2024-08-26

### Added
- [**breaking**] use `SecondElementIs` to verify method.

### Fixed
- [**breaking**] remove unnecessary `#[method_helper]`

### Other
- remove method_helper

## [0.2.0](https://github.com/ratnaraj7/better-routes/compare/better-routes-macros-v0.1.1...better-routes-macros-v0.2.0) - 2024-08-21

### Added
- [**breaking**] use path of struct instead of struct

### Fixed
- add error for #[method_helper] with args
- make diff method handlers pub

### Other
- check reserve methods

## [0.1.1](https://github.com/ratnaraj7/better-routes/compare/better-routes-macros-v0.1.0...better-routes-macros-v0.1.1) - 2024-07-30

### Other
- add repo
- release

## [0.1.0](https://github.com/ratnaraj7/better-routes/releases/tag/better-routes-macros-v0.1.0) - 2024-07-30

### Added
- add method helper macro
- introduce `routes` macro

### Fixed
- add extra_traits features for build to work
- rejection and state to path
- remove imports

### Other
- Update README.md
- add licenses
- add features full syn
- add desc and license
- add readmes
- initial commit
