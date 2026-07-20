# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.2](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.6.1...rsjson-lua-v0.6.2) - 2026-07-20

### Fixed

- *(deps)* update rust crate serde to 1.0.229 ([#67](https://github.com/benniekiss/rs-mod-lua/pull/67))

### Other

- cleanup some formatting

## [0.6.1](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.6.0...rsjson-lua-v0.6.1) - 2026-07-06

### Fixed

- *(rsjson-lua)* fixups for mlua v0.12
- *(deps)* update rust crate mlua to 0.12.0 ([#53](https://github.com/benniekiss/rs-mod-lua/pull/53))

### Other

- bump versions
- *(deps)* update rust to nightly-2026-07-05 ([#21](https://github.com/benniekiss/rs-mod-lua/pull/21))

## [0.6.0](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.5.0...rsjson-lua-v0.6.0) - 2026-06-30

### Fixed

- *(rsjson-lua)* fix tests
- *(rsjson-lua)* use correct `array_metatable` field

### Other

- bump versions
- formatting with luafmt
- *(rsjson-lua)* [**breaking**] change `array_mt` var name to `array_metatable`
- *(rsjson-lua)* some more style refactors
- *(rsjson-lua)* remove unnecessary `.to_serializable()`
- *(rsjson-lua)* make benchmarks more rigorous
- *(rsjson-lua)* refactor some code

## [0.5.0](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.4.2...rsjson-lua-v0.5.0) - 2026-06-23

### Other

- *(rsjson-lua)* cleanup packaging config
- *(rsjson-lua)* [**breaking**] update config bindings

## [0.4.2](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.4.1...rsjson-lua-v0.4.2) - 2026-06-22

### Fixed

- *(windows)* set `panic = 'unwind'` for better windows support

### Other

- bump version
- *(rsjson-lua)* disable benchmark test deps
- add luajit/luajit52 targets

## [0.4.1](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.4.0...rsjson-lua-v0.4.1) - 2026-06-20

### Other

- bump version

## [0.4.0](https://github.com/benniekiss/rs-mod-lua/compare/rsjson-lua-v0.3.5...rsjson-lua-v0.4.0) - 2026-06-19

### Added

- *(rsjson-lua)* expose `array_metatable` to lua
- *(rsjson-lua)* [**breaking**] migrate to `#[derive(mlua::UserData)]`
- [**breaking**] update to mlua v0.12

### Fixed

- *(rsjson-lua)* fix type definitions
- *(rsjson-lua)* build with `opt-level = 3`
- *(rsjson-lua)* use correct imports in benchmarks

### Other

- bump package versions
- *(rsjson-lua)* take arg as `BorrowedBytes`
- *(rsjson-lua)* update benchmarks
- *(rsjson-lua)* more rigorous benchmarks
- *(rsjson-lua)* more rigorous benchmarks
- *(rsjson-lua)* simpler typing
- *(rsjson-lua)* reorganize branching in `encode`
- *(rsjson-lua)* cleanup return types
- *(rsjson-lua)* add benchmark deps to test dependencies
- *(rsjson-lua)* micro-optimize for no encode config

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
