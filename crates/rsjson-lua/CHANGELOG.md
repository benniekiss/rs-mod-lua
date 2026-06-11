# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.5](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.3.4...rsjson-lua-v0.3.5) - 2026-06-11

### Other

- bump lux project version
- release ([#13](https://github.com/benniekiss/rs-mod-lua/pull/13))
- bump lux project version
- Revert "chore: bump lux project versions"

## [0.3.4](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.3.3...rsjson-lua-v0.3.4) - 2026-06-11

### Other

- bump lux project version
- Revert "chore: bump lux project versions"

## [0.3.3](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.3.2...rsjson-lua-v0.3.3) - 2026-06-01

### Fixed

- set the correct `source.dir` value
- update lux.toml with cargo args

### Other

- bump version

## [0.3.2](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.3.1...rsjson-lua-v0.3.2) - 2026-06-01

### Fixed

- remove conflicting `tag` key

### Other

- bump version

## [0.3.1](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.3.0...rsjson-lua-v0.3.1) - 2026-05-31

### Fixed

- set package versions in lux.toml
- prefix version var with `v`

### Other

- bump package versions
- use $(PACKAGE) var in `source.dir`

## [0.3.0](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.2.3...rsjson-lua-v0.3.0) - 2026-05-31

### Added

- move arbitrary_precision support to a feature

### Fixed

- *(rsjson-lua)* fix types path in docs
- return rsjson types table
- *(ci)* restore build.rs file

### Other

- prep for publishing
- update package descriptions [skip ci]
- remove a lot of `use` cruft
- [**breaking**] simplify rsjson package
- *(deps)* update rust to nightly-2026-05-30 ([#2](https://github.com/benniekiss/rs-mod-lua/pull/2))
