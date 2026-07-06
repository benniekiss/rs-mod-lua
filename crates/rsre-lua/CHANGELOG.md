# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.3.2...rsre-lua-v0.3.3) - 2026-07-06

### Fixed

- *(deps)* update rust crate mlua to 0.12.0 ([#53](https://github.com/benniekiss/rs-mod-lua/pull/53))

### Other

- bump versions
- *(deps)* update rust to nightly-2026-07-05 ([#21](https://github.com/benniekiss/rs-mod-lua/pull/21))

## [0.3.2](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.3.1...rsre-lua-v0.3.2) - 2026-06-22

### Fixed

- *(windows)* set `panic = 'unwind'` for better windows support

### Other

- bump version
- add luajit/luajit52 targets

## [0.3.1](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.3.0...rsre-lua-v0.3.1) - 2026-06-20

### Other

- bump version

## [0.3.0](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.2.7...rsre-lua-v0.3.0) - 2026-06-19

### Added

- *(rsre-lua)* expose `fancy_regex::escape` function
- feat!(rsre-lua): migrate to `#[derive(mlua::UserData)]`
- [**breaking**] update to mlua v0.12

### Fixed

- *(rsre-lua)* use correct method in tests
- *(rsre-lua)* update captures test

### Other

- bump package versions
- *(rsre-lua)* accept hay as &str, not String
- *(rsre-lua)* remove use .. as
- *(rsre-lua)* cleanup userdata impl

## [0.2.7](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.2.6...rsre-lua-v0.2.7) - 2026-06-11

### Other

- bump lux project version
- release ([#13](https://github.com/benniekiss/rs-mod-lua/pull/13))
- bump lux project version
- Revert "chore: bump lux project versions"

## [0.2.6](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.2.5...rsre-lua-v0.2.6) - 2026-06-11

### Other

- bump lux project version
- Revert "chore: bump lux project versions"

## [0.2.5](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.2.4...rsre-lua-v0.2.5) - 2026-06-01

### Fixed

- set the correct `source.dir` value
- update lux.toml with cargo args

### Other

- bump version

## [0.2.4](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.2.3...rsre-lua-v0.2.4) - 2026-06-01

### Fixed

- remove conflicting `tag` key

### Other

- bump version

## [0.2.3](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.2.2...rsre-lua-v0.2.3) - 2026-05-31

### Added

- move lua modules to a monorepo

### Fixed

- set package versions in lux.toml
- prefix version var with `v`
- *(ci)* restore build.rs file
- configure source.dir and source.tag
- *(ci)* duplicate rust-toolchain.toml
- update package metadata

### Other

- bump package versions
- use $(PACKAGE) var in `source.dir`
- release ([#4](https://github.com/benniekiss/rs-mod-lua/pull/4))
- prep for publishing
- update package descriptions [skip ci]
- *(deps)* update rust to nightly-2026-05-30 ([#2](https://github.com/benniekiss/rs-mod-lua/pull/2))
- duplicate LICENSE.md
- *(deps)* remove test_dependencies table

## [0.2.2](https://github.com/benniekiss/rs-mod-lua/compare/rsre-lua-v0.2.1...rsre-lua-v0.2.2) - 2026-05-31

### Fixed

- *(ci)* restore build.rs file

### Other

- prep for publishing
- update package descriptions [skip ci]
- *(deps)* update rust to nightly-2026-05-30 ([#2](https://github.com/benniekiss/rs-mod-lua/pull/2))
