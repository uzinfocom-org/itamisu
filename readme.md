<p align="center">
    <img src=".github/assets/header.png" alt="Uzinfocom's {Itamisu}">
</p>

<p align="center">
    <h3 align="center">Just a food picker out of desperation.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/uzinfocom-org/itamisu?style=flat&logo=rust&logoColor=ffffff&labelColor=242424&color=242424" alt="Top Used Language">
    <a href="https://github.com/uzinfocom-org/itamisu/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/uzinfocom-org/itamisu/test.yml?style=flat&logo=github&logoColor=ffffff&labelColor=242424&color=242424" alt="Test CI"></a>
</p>

# About

Sometimes, we just don't know whether what we would like to eat for lunch. So, we build a random food picker
that will choose one for us.

## Development

The project has `shell.nix` which has development environment preconfigured already for you. Just open your
terminal and at the root of this project:

```bash
# Open in bash by default
nix develop

# If you want other shell
nix develop -c $SHELL

# After entering Nix development environment,
# inside the env, you can open your editor, so
# your editor will read all $PATH and environmental
# variables, also your terminal inside your editor
# will adopt all variables, so, you can close terminal.

# Neovim
vim .

# VSCode
code .

# Zed Editor
zed .
```

The development environment has whatever you may need already, but feel free to add or remove whatever
inside `shell.nix`.

## Building

Well, there are two ways of building your project. You can either go with classic `cargo build` way, but before that, make sure to enter development environment to have cargo and all rust toolchain available in your PATH, you may do like that:

```bash
# Entering development environment
nix develop -c $SHELL

# Compile the project
cargo build --release
```

Or, you can build your project via nix which will do all the dirty work for you. Just, in your terminal:

```bash
# Build in nix environment
nix build

# Executable binary is available at:
./result/bin/itamisu
```

## FAQ

### Why not use default.nix for devShell?

There's been cases when I wanted to reproduce totally different behaviors in development environment and
production build. This occurs quite a lot lately for some reason and because of that, I tend to keep
both shell.nix and default.nix to don't mix things up.

[Bleur Stack]: https://github.com/bleur-org

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](license) file for details.

<p align="center">
    <img src=".github/assets/footer.png" alt="Uzinfocom's {Itamisu}">
</p>
