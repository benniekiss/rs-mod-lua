# Installation

Modules are published to
[`luarocks.org`](https://luarocks.org/modules/benniekiss) and
[`crates.io`](https://crates.io/users/benniekiss). They can be installed with
`lux` or `luarocks`.

The most recent version of each module is prebuilt and hosted at
[`https://benniekiss.github.io/rocks/`](https://benniekiss.github.io/rocks/).
The prebuilt binaries can be installed by passing the `--extra-servers
https://benniekiss.github.io/rocks/` to `lx` or `luarocks`

To compile from source, see [building](contributing.md#building)

````{tab} lux
```shell
lx --extra-servers https://benniekiss.github.io/rocks/ install ...
```
````

````{tab} luarocks
```shell
luarocks --extra-servers https://benniekiss.github.io/rocks/ install ...
```
````
