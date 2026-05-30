# Contributing

Contributions are welcome! Feel free to open bug reports, feature requests, or
PRs.

## Code of Conduct

This project adopts the [Rust Code of
Conduct](https://rust-lang.org/policies/code-of-conduct)

## AI Policy

This project adopts the [Linux Kernel AI Coding Assitant
Guidelines](https://docs.kernel.org/process/coding-assistants.html)

In addition, the following policies apply:

AI is not allowed for communication, such as PR or issue report body content. If
you use AI in communication, you may be blocked or banned from contributing to
the project.

Commits should not include `Co-authored-by:` attributions to AI coding tools. If
there is an urge to credit the machine, use the `Assisted-by:` attribution
outlined in the Linux Kernel guidelines.

## Getting Started

This project uses the nightly rust toolchain and the
[lux](https://github.com/lumen-oss/lux) package manager for development.
Pre-commit hooks are provided through [`prek`](https://github.com/j178/prek).

If you do not already have rust installed, you can find [directions
here](https://rust-lang.org/tools/install/)

To install `lux`, follow the [directions
here](https://lux.lumen-labs.org/tutorial/getting-started)

To install `prek`, follow the [directions
here](https://prek.j178.dev/installation/)

### Setup

```shell
# Clone the repo
git clone https://github.com/benniekiss/rslua
cd rslua

# Install the toolchain for the project
rustup install

# If you need to install lux
cargo install lux-cli
lx sync

# If you need to install prek
cargo install prek
prek install-hooks
```

## Linting and Formatting

Make sure to run `cargo fmt`, `cargo check`, and `cargo clippy` prior to any
submissions.

## Tests

Lua tests are within the `spec/` directory for each crate and use
[`busted`](https://github.com/lunarmodules/busted). The tests can be run with
`lx test`

Rust unit tests can be run with `cargo test`.

## Building

To build the project as a lua module, run `lx build`, or `cargo build
--no-default-features --features module,lua{version}`, where `{version}` is one
of `55`, `54`, `53`, `52`, or `51`.

To build it as a library, run `cargo build`.

## Docs

Documentation is generated with
[`sphinx`](https://www.sphinx-doc.org/en/master/) using the
[`sphinx-lua-ls`](https://github.com/sphinx-contrib/lua-ls/) extension.

To generate the docs, [install
uv](https://docs.astral.sh/uv/getting-started/installation/) and run `uvx
--with-requirements=docs/requirements.txt sphinx-autobuild --ignore docs/api
docs/ _site`. The site will be available at `http://127.0.0.1:8000`

## Submitting PRs

Please follow the [Conventional Commits](https://www.conventionalcommits.org)
specification for PR titles.
