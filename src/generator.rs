use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn create_project(name: &str, lang_id: &str, template_id: Option<&str>) -> Result<()> {
    let lang = crate::templates::find_lang(lang_id)
        .ok_or_else(|| anyhow::anyhow!("Unknown language: {lang_id}"))?;

    if let Some(tid) = template_id {
        if crate::templates::find_template(tid, lang_id).is_none() {
            anyhow::bail!("Unknown template '{tid}' for language '{lang_id}'");
        }
    }

    let project_dir = Path::new(name);
    if project_dir.exists() {
        anyhow::bail!("Directory '{name}' already exists");
    }

    fs::create_dir(name)?;
    fs::create_dir(format!("{name}/src"))?;

    generate_common_files(name)?;

    // Base language files
    let mut files = generate_lang_files(name, lang_id);

    // Template overrides
    if let Some(tid) = template_id {
        let tpl_files = generate_template_files(name, lang_id, tid);
        for (path, content) in tpl_files {
            if let Some(pos) = files.iter().position(|(p, _)| p == &path) {
                files[pos] = (path, content);
            } else {
                files.push((path, content));
            }
        }
    }

    for (path, content) in &files {
        let full = format!("{name}/{path}");
        if let Some(parent) = Path::new(&full).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full, content).with_context(|| format!("Failed to write {path}"))?;
    }

    // Packages: base + template extras + always git
    let mut packages: Vec<&str> = lang.packages.to_vec();
    if let Some(tid) = template_id {
        if let Some(t) = crate::templates::find_template(tid, lang_id) {
            packages.extend(t.extra_packages);
        }
    }
    packages.push("git");

    let flake = make_flake(name, &packages);
    fs::write(format!("{name}/flake.nix"), flake)?;

    Ok(())
}

fn generate_common_files(name: &str) -> Result<()> {
    fs::write(format!("{name}/.envrc"), "use flake\n")?;
    fs::write(
        format!("{name}/README.md"),
        format!("# {name}\n\nGenerated with forge\n"),
    )?;
    Ok(())
}

// ----------------------------------------------------------------
// Language dispatcher
// ----------------------------------------------------------------

fn generate_lang_files(name: &str, lang_id: &str) -> Vec<(String, String)> {
    match lang_id {
        "c" => gen_c_base(name),
        "cpp" => gen_cpp_base(name),
        "go" => gen_go_base(name),
        "haskell" => gen_haskell_base(name),
        "lua" => gen_lua_base(name),
        "nodejs" => gen_nodejs_base(name),
        "ocaml" => gen_ocaml_base(name),
        "python" => gen_python_base(name),
        "rust" => gen_rust_base(name),
        "typescript" => gen_typescript_base(name),
        "zig" => gen_zig_base(name),
        _ => unreachable!(),
    }
}

// ----------------------------------------------------------------
// Template dispatcher
// ----------------------------------------------------------------

fn generate_template_files(name: &str, lang_id: &str, template_id: &str) -> Vec<(String, String)> {
    match (lang_id, template_id) {
        ("c", "allegro") => gen_c_tpl(name, ALLEGRO_MAIN, "$(shell pkg-config --cflags allegro-5)", "$(shell pkg-config --libs allegro-5)"),
        ("c", "cairo")   => gen_c_tpl(name, CAIRO_MAIN,   "$(shell pkg-config --cflags cairo)",   "$(shell pkg-config --libs cairo)"),
        ("c", "curl")    => gen_c_tpl(name, CURL_MAIN,    "$(shell pkg-config --cflags libcurl)", "$(shell pkg-config --libs libcurl)"),
        ("c", "gtk3")    => gen_c_tpl(name, GTK3_MAIN,    "$(shell pkg-config --cflags gtk+-3.0)", "$(shell pkg-config --libs gtk+-3.0)"),
        ("c", "ncurses") => gen_c_tpl(name, NCURSES_MAIN, "", "-lncurses"),
        ("c", "opengl")  => gen_c_tpl(name, OPENGL_MAIN,  "$(shell pkg-config --cflags glfw3 gl)", "$(shell pkg-config --libs glfw3 gl)"),
        ("c", "raylib")  => gen_c_tpl(name, RAYLIB_MAIN,  "$(shell pkg-config --cflags raylib)",  "$(shell pkg-config --libs raylib)"),
        ("c", "sdl2")    => gen_c_tpl(name, SDL2_MAIN,    "$(shell pkg-config --cflags sdl2)",    "$(shell pkg-config --libs sdl2)"),
        ("c", "xlib")    => gen_c_tpl(name, XLIB_MAIN,    "$(shell pkg-config --cflags x11)",     "$(shell pkg-config --libs x11)"),
        _ => unreachable!(),
    }
}

fn gen_c_tpl(name: &str, main_src: &str, cflags: &str, ldflags: &str) -> Vec<(String, String)> {
    vec![
        ("src/main.c".into(), main_src.replace("{name}", name)),
        ("Makefile".into(), makefile_c(name, cflags, ldflags)),
    ]
}

// ----------------------------------------------------------------
// flake.nix generation
// ----------------------------------------------------------------

fn make_flake(name: &str, packages: &[&str]) -> String {
    let pkgs: Vec<String> = packages
        .iter()
        .map(|p| format!("            {p}"))
        .collect();

    r#"# This file was generated by forge
{
  description = "{name}";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
{PACKAGES}
          ];
        };
      });
}
"#
    .replace("{name}", name)
    .replace("{PACKAGES}", &pkgs.join("\n"))
}

// ----------------------------------------------------------------
// Makefile helpers
// ----------------------------------------------------------------

fn makefile_c(name: &str, extra_cflags: &str, extra_ldflags: &str) -> String {
    let cflags = if extra_cflags.is_empty() {
        String::from("CFLAGS = -Wall -Wextra -O2 -std=c11")
    } else {
        format!("CFLAGS = -Wall -Wextra -O2 -std=c11 {extra_cflags}")
    };
    let ldflags = if extra_ldflags.is_empty() {
        String::from("LDFLAGS =")
    } else {
        format!("LDFLAGS = {extra_ldflags}")
    };

    r#"BUILD_DIR = build
BIN = $(BUILD_DIR)/{name}

CC = gcc
{CFLAGS_LINE}
{LDFLAGS_LINE}
SRC = src/main.c

all: $(BIN)

$(BIN): $(SRC) | $(BUILD_DIR)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

$(BUILD_DIR):
	mkdir -p $@

run: $(BIN)
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR)
.PHONY: all run clean
"#
    .replace("{name}", name)
    .replace("{CFLAGS_LINE}", &cflags)
    .replace("{LDFLAGS_LINE}", &ldflags)
}

fn makefile_cpp(name: &str, extra_cxxflags: &str, extra_ldflags: &str) -> String {
    let cxxflags = if extra_cxxflags.is_empty() {
        String::from("CXXFLAGS = -Wall -Wextra -O2 -std=c++17")
    } else {
        format!("CXXFLAGS = -Wall -Wextra -O2 -std=c++17 {extra_cxxflags}")
    };
    let ldflags = if extra_ldflags.is_empty() {
        String::from("LDFLAGS =")
    } else {
        format!("LDFLAGS = {extra_ldflags}")
    };

    r#"BUILD_DIR = build
BIN = $(BUILD_DIR)/{name}

CXX = g++
{CXXFLAGS_LINE}
{LDFLAGS_LINE}
SRC = src/main.cpp

all: $(BIN)

$(BIN): $(SRC) | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) -o $@ $^ $(LDFLAGS)

$(BUILD_DIR):
	mkdir -p $@

run: $(BIN)
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR)
.PHONY: all run clean
"#
    .replace("{name}", name)
    .replace("{CXXFLAGS_LINE}", &cxxflags)
    .replace("{LDFLAGS_LINE}", &ldflags)
}

// ----------------------------------------------------------------
// Language base generators
// ----------------------------------------------------------------

fn gen_c_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/main.c".into(), C_BARE_MAIN.replace("{name}", name)),
        ("Makefile".into(), makefile_c(name, "", "")),
    ]
}

fn gen_cpp_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/main.cpp".into(), CPP_MAIN.replace("{name}", name)),
        ("Makefile".into(), makefile_cpp(name, "", "")),
    ]
}

fn gen_go_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/main.go".into(), GO_MAIN.replace("{name}", name)),
        ("go.mod".into(), GO_MOD.replace("{name}", name)),
        ("Makefile".into(), GO_MAKEFILE.replace("{name}", name)),
    ]
}

fn gen_haskell_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/Main.hs".into(), HASKELL_MAIN.replace("{name}", name)),
        ("Makefile".into(), HASKELL_MAKEFILE.replace("{name}", name)),
    ]
}

fn gen_lua_base(name: &str) -> Vec<(String, String)> {
    vec![(
        "src/main.lua".into(),
        LUA_MAIN.replace("{name}", name),
    )]
}

fn gen_nodejs_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/index.js".into(), NODEJS_MAIN.replace("{name}", name)),
        ("package.json".into(), NODEJS_PKG.replace("{name}", name)),
    ]
}

fn gen_ocaml_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/main.ml".into(), OCAML_MAIN.replace("{name}", name)),
        ("Makefile".into(), OCAML_MAKEFILE.replace("{name}", name)),
    ]
}

fn gen_python_base(name: &str) -> Vec<(String, String)> {
    vec![(
        "src/main.py".into(),
        PYTHON_MAIN.replace("{name}", name),
    )]
}

fn gen_rust_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/main.rs".into(), RUST_MAIN.replace("{name}", name)),
        ("Cargo.toml".into(), RUST_CARGO_TOML.replace("{name}", name)),
    ]
}

fn gen_typescript_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/index.ts".into(), TYPESCRIPT_MAIN.replace("{name}", name)),
        ("package.json".into(), TYPESCRIPT_PKG.replace("{name}", name)),
        ("tsconfig.json".into(), TYPESCRIPT_TSCONFIG.to_string()),
    ]
}

fn gen_zig_base(name: &str) -> Vec<(String, String)> {
    vec![
        ("src/main.zig".into(), ZIG_MAIN.replace("{name}", name)),
        ("Makefile".into(), ZIG_MAKEFILE.replace("{name}", name)),
    ]
}

// ----------------------------------------------------------------
// Inline source templates
// ----------------------------------------------------------------

// --- C ---
const C_BARE_MAIN: &str = r#"#include <stdio.h>

int main(void) {
    printf("Hello from {name}!\n");
    return 0;
}
"#;

// --- C++ ---
const CPP_MAIN: &str = r#"#include <iostream>

int main() {
    std::cout << "Hello from {name}!" << std::endl;
    return 0;
}
"#;

// --- Go ---
const GO_MAIN: &str = r#"package main

import "fmt"

func main() {
    fmt.Println("Hello from {name}!")
}
"#;

const GO_MOD: &str = r#"module {name}

go 1.22
"#;

const GO_MAKEFILE: &str = r#"BUILD_DIR = build
BIN = $(BUILD_DIR)/{name}

all: $(BIN)

$(BIN): src/main.go go.mod | $(BUILD_DIR)
	go build -o $@ ./src

$(BUILD_DIR):
	mkdir -p $@

run: $(BIN)
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR)
.PHONY: all run clean
"#;

// --- Haskell ---
const HASKELL_MAIN: &str = r#"module Main where

main :: IO ()
main = putStrLn "Hello from {name}!"
"#;

const HASKELL_MAKEFILE: &str = r#"BUILD_DIR = build
BIN = $(BUILD_DIR)/{name}

all: $(BIN)

$(BIN): src/Main.hs | $(BUILD_DIR)
	ghc -o $@ $<

$(BUILD_DIR):
	mkdir -p $@

run: $(BIN)
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR) *.hi *.o
.PHONY: all run clean
"#;

// --- Lua ---
const LUA_MAIN: &str = r#"print("Hello from {name}!")
"#;

// --- Node.js ---
const NODEJS_MAIN: &str = r#"const http = require('http');

const hostname = '127.0.0.1';
const port = 3000;

const server = http.createServer((req, res) => {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/plain');
    res.end('Hello from {name}!\n');
});

server.listen(port, hostname, () => {
    console.log(`Server running at http://${hostname}:${port}/`);
});
"#;

const NODEJS_PKG: &str = r#"{
  "name": "{name}",
  "version": "1.0.0",
  "private": true,
  "main": "src/index.js",
  "scripts": {
    "start": "node src/index.js"
  }
}
"#;

// --- OCaml ---
const OCAML_MAIN: &str = r#"let () = print_endline "Hello from {name}!"
"#;

const OCAML_MAKEFILE: &str = r#"BUILD_DIR = build
BIN = $(BUILD_DIR)/{name}

all: $(BIN)

$(BIN): src/main.ml | $(BUILD_DIR)
	ocamlopt -o $@ $<

$(BUILD_DIR):
	mkdir -p $@

run: $(BIN)
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR) *.cmx *.o
.PHONY: all run clean
"#;

// --- Python ---
const PYTHON_MAIN: &str = r#"def main():
    print("Hello from {name}!")


if __name__ == "__main__":
    main()
"#;

// --- Rust ---
const RUST_MAIN: &str = r#"fn main() {
    println!("Hello from {name}!");
}
"#;

const RUST_CARGO_TOML: &str = r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
"#;

// --- TypeScript ---
const TYPESCRIPT_MAIN: &str = r#"function main(): void {
    console.log(`Hello from {name}!`);
}

main();
"#;

const TYPESCRIPT_PKG: &str = r#"{
  "name": "{name}",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "build": "npx tsc",
    "start": "node dist/index.js"
  },
  "devDependencies": {
    "typescript": "^5.0.0"
  }
}
"#;

const TYPESCRIPT_TSCONFIG: &str = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "esModuleInterop": true
  },
  "include": ["src/**/*"]
}
"#;

// --- Zig ---
const ZIG_MAIN: &str = r#"const std = @import("std");

pub fn main() void {
    std.debug.print("Hello from {s}!\n", .{"{name}"});
}
"#;

const ZIG_MAKEFILE: &str = r#"BUILD_DIR = build
BIN = $(BUILD_DIR)/{name}

all: $(BIN)

$(BIN): src/main.zig | $(BUILD_DIR)
	zig build-exe $< -femit-bin=$@

$(BUILD_DIR):
	mkdir -p $@

run: $(BIN)
	./$(BIN)

clean:
	rm -rf $(BUILD_DIR)
.PHONY: all run clean
"#;

// ----------------------------------------------------------------
// C template source files
// ----------------------------------------------------------------

const ALLEGRO_MAIN: &str = r#"#include <allegro5/allegro.h>

int main(void) {
    if (!al_init()) return -1;

    ALLEGRO_DISPLAY *d = al_create_display(800, 600);
    if (!d) return -1;

    al_set_window_title(d, "{name}");
    al_clear_to_color(al_map_rgb(24, 24, 24));
    al_flip_display();
    al_rest(3.0);

    al_destroy_display(d);
    al_uninstall_system();
    return 0;
}
"#;

const CAIRO_MAIN: &str = r#"#include <cairo.h>

int main(void) {
    cairo_surface_t *s = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 200, 200);
    cairo_t *cr = cairo_create(s);

    cairo_set_source_rgb(cr, 0.1, 0.1, 0.1);
    cairo_paint(cr);

    cairo_set_source_rgb(cr, 1, 0, 0);
    cairo_move_to(cr, 100, 40);
    cairo_line_to(cr, 40, 160);
    cairo_line_to(cr, 160, 160);
    cairo_close_path(cr);
    cairo_fill(cr);

    cairo_surface_write_to_png(s, "{name}.png");
    cairo_destroy(cr);
    cairo_surface_destroy(s);
    return 0;
}
"#;

const CURL_MAIN: &str = r#"#include <stdio.h>
#include <curl/curl.h>

static size_t write_cb(void *d, size_t s, size_t n, void *u) {
    return fwrite(d, s, n, stdout);
}

int main(void) {
    curl_global_init(CURL_GLOBAL_DEFAULT);
    CURL *c = curl_easy_init();
    if (!c) return 1;

    curl_easy_setopt(c, CURLOPT_URL, "https://httpbin.org/get");
    curl_easy_setopt(c, CURLOPT_WRITEFUNCTION, write_cb);

    CURLcode r = curl_easy_perform(c);
    if (r != CURLE_OK)
        fprintf(stderr, "curl: %s\n", curl_easy_strerror(r));

    curl_easy_cleanup(c);
    curl_global_cleanup();
    return 0;
}
"#;

const GTK3_MAIN: &str = r#"#include <gtk/gtk.h>

static void act(GtkApplication *a, gpointer d) {
    GtkWidget *w = gtk_application_window_new(a);
    gtk_window_set_title(GTK_WINDOW(w), "{name}");
    gtk_window_set_default_size(GTK_WINDOW(w), 400, 300);
    gtk_widget_show_all(w);
}

int main(int argc, char **argv) {
    GtkApplication *a = gtk_application_new("com.example.{name}", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(a, "activate", G_CALLBACK(act), NULL);
    int s = g_application_run(G_APPLICATION(a), argc, argv);
    g_object_unref(a);
    return s;
}
"#;

const NCURSES_MAIN: &str = r#"#include <ncurses.h>

int main(void) {
    initscr();
    cbreak();
    noecho();
    clear();
    mvprintw(10, 10, "Hello from {name}!");
    refresh();
    getch();
    endwin();
    return 0;
}
"#;

const OPENGL_MAIN: &str = r#"#include <GLFW/glfw3.h>
#include <GL/gl.h>

int main(void) {
    if (!glfwInit()) return -1;

    GLFWwindow *w = glfwCreateWindow(800, 600, "{name}", NULL, NULL);
    if (!w) { glfwTerminate(); return -1; }
    glfwMakeContextCurrent(w);

    while (!glfwWindowShouldClose(w)) {
        glClear(GL_COLOR_BUFFER_BIT);
        glBegin(GL_TRIANGLES);
        glColor3f(1,0,0); glVertex2f(-0.5f,-0.5f);
        glColor3f(0,1,0); glVertex2f( 0.5f,-0.5f);
        glColor3f(0,0,1); glVertex2f( 0.0f, 0.5f);
        glEnd();
        glfwSwapBuffers(w);
        glfwPollEvents();
    }

    glfwDestroyWindow(w);
    glfwTerminate();
    return 0;
}
"#;

const RAYLIB_MAIN: &str = r#"#include "raylib.h"

int main(void) {
    const int w = 800, h = 450;
    InitWindow(w, h, "{name}");
    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        BeginDrawing();
        ClearBackground(RAYWHITE);
        DrawText("Hello from {name}!", 190, 200, 20, LIGHTGRAY);
        EndDrawing();
    }

    CloseWindow();
    return 0;
}
"#;

const SDL2_MAIN: &str = r#"#include <SDL2/SDL.h>

int main(void) {
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window *w = SDL_CreateWindow("{name}", SDL_WINDOWPOS_UNDEFINED,
                                     SDL_WINDOWPOS_UNDEFINED, 800, 600,
                                     SDL_WINDOW_SHOWN);
    SDL_Surface *s = SDL_GetWindowSurface(w);
    SDL_FillRect(s, NULL, SDL_MapRGB(s->format, 0xFF, 0xFF, 0xFF));
    SDL_UpdateWindowSurface(w);
    SDL_Delay(3000);
    SDL_DestroyWindow(w);
    SDL_Quit();
    return 0;
}
"#;

const XLIB_MAIN: &str = r#"#include <X11/Xlib.h>

int main(void) {
    Display *d = XOpenDisplay(NULL);
    if (!d) return 1;

    int s = DefaultScreen(d);
    Window w = XCreateSimpleWindow(d, RootWindow(d,s), 0, 0, 400, 300, 1,
                                    BlackPixel(d,s), WhitePixel(d,s));
    XSelectInput(d, w, ExposureMask | KeyPressMask);
    XMapWindow(d, w);

    GC gc = DefaultGC(d, s);
    XEvent e;
    while (1) {
        XNextEvent(d, &e);
        if (e.type == Expose)
            XDrawString(d, w, gc, 120, 150, "Hello from {name}!", 18);
        if (e.type == KeyPress) break;
    }

    XCloseDisplay(d);
    return 0;
}
"#;

// ----------------------------------------------------------------
// Add packages to existing flake.nix
// ----------------------------------------------------------------

pub fn add_packages(name: &str, packages: &[String]) -> Result<()> {
    let flake_path = format!("{name}/flake.nix");
    let content =
        fs::read_to_string(&flake_path).with_context(|| format!("Failed to read {flake_path}"))?;

    let marker = "buildInputs = with pkgs; [";
    let marker_pos = content
        .find(marker)
        .ok_or_else(|| anyhow::anyhow!("Could not find buildInputs in flake.nix"))?;

    let after_marker = &content[marker_pos + marker.len()..];
    let close_pos = after_marker
        .find("];")
        .ok_or_else(|| anyhow::anyhow!("Could not find closing ] in buildInputs"))?;
    let close_abs = marker_pos + marker.len() + close_pos;

    let insert_pos = content[..close_abs]
        .rfind('\n')
        .map(|p| p + 1)
        .unwrap_or(close_abs);

    let existing = &content[marker_pos + marker.len()..insert_pos];
    let to_add: Vec<&String> = packages
        .iter()
        .filter(|p| !existing.contains(p.as_str()))
        .collect();

    if to_add.is_empty() {
        println!("All packages already present");
        return Ok(());
    }

    let indent = "            ";
    let mut new_content = String::new();
    new_content.push_str(&content[..insert_pos]);
    for pkg in &to_add {
        new_content.push_str(&format!("{indent}{pkg}\n"));
    }
    new_content.push_str(&content[insert_pos..]);

    fs::write(&flake_path, new_content)?;
    let added: Vec<&str> = to_add.iter().map(|s| s.as_str()).collect();
    println!("Added packages: {}", added.join(", "));
    Ok(())
}
