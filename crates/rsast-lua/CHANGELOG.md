# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/benniekiss/rs-mod-lua/compare/rsast-lua-v0.1.3...rsast-lua-v0.2.0) - 2026-07-22

### Added

- *(rsast-lua)* allow configuring call limit and error reporting

### Fixed

- *(rsast-lua)* use optional return type on Pair:pairs()

### Other

- add keywords and categories to Cargo.toml
- *(rsast-lua)* [**breaking**] bump version
- *(rsast-lua)* replace Rc with Arc

## [0.1.3](https://github.com/benniekiss/rs-mod-lua/compare/rsast-lua-v0.1.2...rsast-lua-v0.1.3) - 2026-07-22

### Added

- *(rsast-lua)* add `find_tagged` method

### Fixed

- *(deps)* update pest to 2.8.8 ([#86](https://github.com/benniekiss/rs-mod-lua/pull/86))
- *(rsast-lua)* mark `as_node_tag` as optional return

### Other

- *(rsast-lua)* bump version
- *(rsast-lua)* refactor `flatten` methods

## [0.1.2](https://github.com/benniekiss/rs-mod-lua/compare/rsast-lua-v0.1.1...rsast-lua-v0.1.2) - 2026-07-21

### Other

- *(rsast-lua)* more type def cleanup
- *(rsast-lua)* bump version
- *(rsast-lua)* update typedefs

## [0.1.1](https://github.com/benniekiss/rs-mod-lua/compare/rsast-lua-v0.1.0...rsast-lua-v0.1.1) - 2026-07-20

### Fixed

- *(deps)* update rust crate serde to 1.0.229 ([#77](https://github.com/benniekiss/rs-mod-lua/pull/77))

### Other

- *(deps)* update rust to nightly-2026-07-20 ([#76](https://github.com/benniekiss/rs-mod-lua/pull/76))
- *(rsast-lua)* bump version
