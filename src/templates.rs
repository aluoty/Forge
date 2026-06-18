use dialoguer::Select;

pub struct Template {
    pub id: &'static str,
    pub label: &'static str,
    pub desc: &'static str,
    pub packages: &'static [&'static str],
}

pub static TEMPLATES: &[Template] = &[
    Template {
        id: "c",
        label: "C (bare)",
        desc: "A minimal C project with GCC and Make",
        packages: &["gcc", "gnumake", "gdb"],
    },
    Template {
        id: "c-allegro",
        label: "C with Allegro",
        desc: "C project with Allegro for game development",
        packages: &["gcc", "gnumake", "gdb", "pkg-config", "allegro"],
    },
    Template {
        id: "c-curl",
        label: "C with cURL",
        desc: "C project with libcurl for HTTP requests",
        packages: &["gcc", "gnumake", "gdb", "pkg-config", "curl"],
    },
    Template {
        id: "c-gtk3",
        label: "C with GTK3",
        desc: "C project with GTK3 for desktop GUIs",
        packages: &["gcc", "gnumake", "gdb", "pkg-config", "gtk3"],
    },
    Template {
        id: "c-ncurses",
        label: "C with Ncurses",
        desc: "C project with Ncurses for terminal UIs",
        packages: &["gcc", "gnumake", "gdb", "ncurses"],
    },
    Template {
        id: "c-opengl",
        label: "C with OpenGL (GLFW)",
        desc: "C project with OpenGL, GLFW, and GLEW",
        packages: &["gcc", "gnumake", "gdb", "pkg-config", "libGL", "glfw", "glew"],
    },
    Template {
        id: "c-raylib",
        label: "C with Raylib",
        desc: "C project with Raylib for graphics",
        packages: &["gcc", "gnumake", "gdb", "pkg-config", "raylib"],
    },
    Template {
        id: "c-sdl2",
        label: "C with SDL2",
        desc: "C project with SDL2 for multimedia",
        packages: &["gcc", "gnumake", "gdb", "pkg-config", "SDL2", "SDL2_image", "SDL2_mixer", "SDL2_ttf"],
    },
    Template {
        id: "cpp",
        label: "C++ (bare)",
        desc: "A minimal C++ project with G++ and Make",
        packages: &["gcc", "gnumake", "gdb"],
    },
    Template {
        id: "go",
        label: "Go",
        desc: "A Go project",
        packages: &["go", "gopls"],
    },
    Template {
        id: "nodejs",
        label: "Node.js",
        desc: "A Node.js project",
        packages: &["nodejs"],
    },
    Template {
        id: "python",
        label: "Python",
        desc: "A Python project",
        packages: &["python3", "pip"],
    },
    Template {
        id: "rust",
        label: "Rust",
        desc: "A Rust project with Cargo",
        packages: &["cargo", "rustc"],
    },
    Template {
        id: "zig",
        label: "Zig",
        desc: "A Zig project",
        packages: &["zig"],
    },
];

pub fn find(id: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.id == id)
}

pub fn interactive_select() -> anyhow::Result<String> {
    let items: Vec<String> = TEMPLATES
        .iter()
        .map(|t| format!("{:<14} {}", t.id, t.desc))
        .collect();

    let selection = Select::new()
        .with_prompt("Select a language template")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(TEMPLATES[selection].id.to_string())
}

pub fn list_templates() {
    println!("Available templates:");
    for t in TEMPLATES {
        println!("  {:<14} {} - {}", t.id, t.label, t.desc);
    }
    println!();
    println!("Use: forge new <name> --lang <id>");
}
