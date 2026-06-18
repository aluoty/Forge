# Forge

A project scaffold generator for Nix. Create language-specific projects with proper `flake.nix` dev shells, ready to use with `direnv` and `nix develop`.

## Commands

| Command                          | Description                                        |
|----------------------------------|----------------------------------------------------|
| `forge new <name>`               | Create project with interactive template selection |
| `forge new <name> --lang <id>`   | Create project with a specific template            |
| `forge list`                     | List available language templates                  |
| `forge add <name> <pkgs>...`     | Add Nix packages to an existing project's flake.nix|

## Templates

| ID           | Language / Framework        | Nix Packages                                                        |
|--------------|-----------------------------|---------------------------------------------------------------------|
| `c`          | C (bare)                    | gcc, gnumake, gdb                                                   |
| `c-ncurses`  | C with Ncurses              | gcc, gnumake, gdb, ncurses                                          |
| `c-raylib`   | C with Raylib               | gcc, gnumake, gdb, pkg-config, raylib                               |
| `c-opengl`   | C with OpenGL / GLFW        | gcc, gnumake, gdb, pkg-config, libGL, glfw, glew                    |
| `c-sdl2`     | C with SDL2                 | gcc, gnumake, gdb, pkg-config, SDL2, SDL2_image, SDL2_mixer, SDL2_ttf |
| `rust`       | Rust                        | cargo, rustc                                                        |
| `python`     | Python                      | python3, pip                                                        |

## Example

```bash
# Interactive mode
forge new mygame

# Or non-interactive
forge new mygame --lang c-raylib

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
forge add myproject cmake ninja pkg-config
```

Packages already present are skipped.
