# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.3.0...minijinja-lua-v0.3.1) - 2026-06-21

### Fixed

- *(minijinja-lua)* prevent panics when recursively calling `bind`

## [0.3.0](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.2.1...minijinja-lua-v0.3.0) - 2026-06-21

### Added

- *(minijinja-lua)* [**breaking**] bump version
- remove unsafe usage

### Other

- *(minijinja-lua)* rename struct
- *(minijinja-lua)* encapsulate lua binding logic
- *(minijinja-lua)* remove unnecessary map
- *(minijinja-lua)* move lua code to a dedicated module
- *(minijinja-lua)* refactor some code for readability

## [0.2.1](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.2.0...minijinja-lua-v0.2.1) - 2026-06-20

### Other

- *(minijinja-lua)* add safety comment
- bump version
- *(minijinja-lua)* cleanup types/conversions

## [0.2.0](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.1.27...minijinja-lua-v0.2.0) - 2026-06-19

### Added

- *(minijinja-lua)* migrate Environment to macro userdata
- [**breaking**] update to mlua v0.12

### Fixed

- *(minijinja-lua)* support returning multiple values from lua functions/callbacks ([#25](https://github.com/benniekiss/rs-mod-lua/pull/25))
- *(minijinja-lua)* remove deprecated tests
- *(minijinja-lua)* set correct method name
- *(minijinja-lua)* fix `state` userdata type test

### Other

- bump package versions
- *(minijinja-lua)* implement newtypes for several structs ([#26](https://github.com/benniekiss/rs-mod-lua/pull/26))
- *(minijinja-lua)* bump minijinja version
- *(minijinja-lua)* align lifetime annotation names
- *(minijinja-lua)* cleanup userdata impl

## [0.1.27](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.1.26...minijinja-lua-v0.1.27) - 2026-06-11

### Other

- bump lux project version
- release ([#13](https://github.com/benniekiss/rs-mod-lua/pull/13))
- bump lux project version
- Revert "chore: bump lux project versions"

## [0.1.26](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.1.25...minijinja-lua-v0.1.26) - 2026-06-11

### Other

- bump lux project version
- Revert "chore: bump lux project versions"

## [0.1.25](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.1.24...minijinja-lua-v0.1.25) - 2026-06-01

### Fixed

- set the correct `source.dir` value
- update lux.toml with cargo args

### Other

- bump version

## [0.1.24](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.1.23...minijinja-lua-v0.1.24) - 2026-06-01

### Fixed

- remove conflicting `tag` key

### Other

- bump version

## [0.1.23](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.1.22...minijinja-lua-v0.1.23) - 2026-05-31

### Added

- move lua modules to a monorepo

### Fixed

- set package versions in lux.toml
- prefix version var with `v`
- *(ci)* restore build.rs file
- configure source.dir and source.tag
- lua 5.1 and 5.2 compat in path_loader
- *(ci)* duplicate rust-toolchain.toml
- update package metadata

### Other

- bump package versions
- use $(PACKAGE) var in `source.dir`
- release ([#4](https://github.com/benniekiss/rs-mod-lua/pull/4))
- prep for publishing
- update package descriptions [skip ci]
- *(minijinja-lua)* update jiff crate
- remove use some mlua statements
- *(deps)* update rust to nightly-2026-05-30 ([#2](https://github.com/benniekiss/rs-mod-lua/pull/2))
- duplicate LICENSE.md
- *(deps)* remove test_dependencies table

## [0.1.22](https://github.com/benniekiss/rs-mod-lua/compare/minijinja-lua-v0.1.21...minijinja-lua-v0.1.22) - 2026-05-31

### Fixed

- *(ci)* restore build.rs file

### Other

- prep for publishing
- update package descriptions [skip ci]
- *(minijinja-lua)* update jiff crate
- remove use some mlua statements
- *(deps)* update rust to nightly-2026-05-30 ([#2](https://github.com/benniekiss/rs-mod-lua/pull/2))
