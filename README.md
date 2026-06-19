# Forge

A project scaffold generator for Nix. Create language-specific projects with
proper `flake.nix` dev shells, ready to use with `direnv` and `nix develop`.

## Commands

| Command                                    | Description                                        |
|--------------------------------------------|----------------------------------------------------|
| `forge new <name>`                         | Create project (interactive language selection)    |
| `forge new <name> --lang <lang>`           | Create project with specific language              |
| `forge new <name> --lang <lang> --template <t>` | Create project with language template          |
| `forge list`                               | List available languages and templates             |
| `forge doctor`                             | Check system setup and list everything             |
| `forge add <name> <pkgs>...`               | Add Nix packages to an existing flake.nix          |

## Languages (11)

| Lang         | Description                            | Nix packages               |
|--------------|----------------------------------------|----------------------------|
| `c`          | GCC + Make                             | gcc, gnumake, gdb          |
| `cpp`        | G++ + Make                             | gcc, gnumake, gdb          |
| `go`         | Go toolchain                           | go, gopls                  |
| `haskell`    | GHC                                    | ghc                        |
| `lua`        | Lua interpreter                        | lua                        |
| `nodejs`     | Node.js + npm                          | nodejs                     |
| `ocaml`      | OCaml compiler                         | ocaml                      |
| `python`     | Python 3 + pip                         | python3, pip               |
| `rust`       | Cargo + rustc                          | cargo, rustc               |
| `typescript` | Node.js + TypeScript compiler          | nodejs, typescript         |
| `zig`        | Zig compiler                           | zig                        |

All projects include `git` in their dev shell.

## Templates for C

Use `forge new <name> --lang c --template <name>` to add graphics, GUI, or
networking libraries to a C project:

| Template   | Library         | Description              |
|------------|-----------------|--------------------------|
| `allegro`  | Allegro         | Game development library |
| `cairo`    | Cairo           | 2D vector graphics       |
| `curl`     | libcurl         | HTTP requests            |
| `gtk3`     | GTK3            | Desktop GUI toolkit      |
| `ncurses`  | Ncurses         | Terminal UIs             |
| `opengl`   | OpenGL + GLFW   | 3D graphics              |
| `raylib`   | Raylib          | Simple graphics library  |
| `sdl2`     | SDL2            | Multimedia library       |
| `xlib`     | X11 (Xlib)      | X11 window system        |

## Examples

```bash
# Interactive mode
forge new mygame

# Bare C project
forge new mygame --lang c

# C with Raylib
forge new mygame --lang c --template raylib

# Rust
forge new mycli --lang rust

# Haskell
forge new myapp --lang haskell

# Enter the project
cd mygame
direnv allow   # or: nix develop

# Build and run
make run

# Later, add more Nix packages
forge add mygame cmake ninja
```

## Adding Packages

The `add` command inserts Nix packages into the `buildInputs` of an existing
project's `flake.nix`:

```bash
forge add myproject cmake ninja
```

Packages already present are skipped.

## Doctor

Check what's installed and see everything available:

```bash
forge doctor
```
