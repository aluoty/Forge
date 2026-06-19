use dialoguer::Select;

pub struct Language {
    pub id: &'static str,
    pub label: &'static str,
    pub desc: &'static str,
    pub packages: &'static [&'static str],
}

pub struct Template {
    pub id: &'static str,
    pub label: &'static str,
    pub desc: &'static str,
    pub lang: &'static str,
    pub extra_packages: &'static [&'static str],
}

pub static LANGUAGES: &[Language] = &[
    Language { id: "c", label: "C", desc: "A C project with GCC and Make", packages: &["gcc", "gnumake", "gdb"] },
    Language { id: "cpp", label: "C++", desc: "A C++ project with G++ and Make", packages: &["gcc", "gnumake", "gdb"] },
    Language { id: "go", label: "Go", desc: "A Go project", packages: &["go", "gopls"] },
    Language { id: "haskell", label: "Haskell", desc: "A Haskell project with GHC", packages: &["ghc"] },
    Language { id: "lua", label: "Lua", desc: "A Lua project", packages: &["lua"] },
    Language { id: "nodejs", label: "Node.js", desc: "A Node.js project", packages: &["nodejs"] },
    Language { id: "ocaml", label: "OCaml", desc: "An OCaml project", packages: &["ocaml"] },
    Language { id: "python", label: "Python", desc: "A Python project", packages: &["python3", "pip"] },
    Language { id: "rust", label: "Rust", desc: "A Rust project with Cargo", packages: &["cargo", "rustc"] },
    Language { id: "typescript", label: "TypeScript", desc: "A TypeScript project", packages: &["nodejs", "typescript"] },
    Language { id: "zig", label: "Zig", desc: "A Zig project", packages: &["zig"] },
];

pub static TEMPLATES: &[Template] = &[
    Template { id: "allegro", label: "Allegro", desc: "Game development library", lang: "c", extra_packages: &["pkg-config", "allegro"] },
    Template { id: "cairo", label: "Cairo", desc: "2D vector graphics", lang: "c", extra_packages: &["pkg-config", "cairo"] },
    Template { id: "curl", label: "cURL", desc: "HTTP requests", lang: "c", extra_packages: &["pkg-config", "curl"] },
    Template { id: "gtk3", label: "GTK3", desc: "Desktop GUI toolkit", lang: "c", extra_packages: &["pkg-config", "gtk3"] },
    Template { id: "ncurses", label: "Ncurses", desc: "Terminal UIs", lang: "c", extra_packages: &["ncurses"] },
    Template { id: "opengl", label: "OpenGL (GLFW)", desc: "OpenGL with GLFW", lang: "c", extra_packages: &["pkg-config", "libGL", "glfw", "glew"] },
    Template { id: "raylib", label: "Raylib", desc: "Simple graphics library", lang: "c", extra_packages: &["pkg-config", "raylib"] },
    Template { id: "sdl2", label: "SDL2", desc: "Multimedia library", lang: "c", extra_packages: &["pkg-config", "SDL2"] },
    Template { id: "xlib", label: "Xlib", desc: "X11 window system", lang: "c", extra_packages: &["pkg-config", "xorg.libX11"] },
];

pub fn find_lang(id: &str) -> Option<&'static Language> {
    LANGUAGES.iter().find(|l| l.id == id)
}

pub fn find_template(id: &str, lang: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.id == id && t.lang == lang)
}

pub fn templates_for_lang(lang: &str) -> Vec<&'static Template> {
    TEMPLATES.iter().filter(|t| t.lang == lang).collect()
}

pub fn interactive_select_lang() -> anyhow::Result<String> {
    let items: Vec<String> = LANGUAGES
        .iter()
        .map(|l| format!("{:<14} {}", l.id, l.desc))
        .collect();

    let sel = Select::new()
        .with_prompt("Select a language")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(LANGUAGES[sel].id.to_string())
}

pub fn interactive_select_template(lang: &str) -> anyhow::Result<Option<String>> {
    let tpls = templates_for_lang(lang);
    if tpls.is_empty() {
        return Ok(None);
    }

    let use_tpl = match dialoguer::Confirm::new()
        .with_prompt("Use a template?")
        .default(false)
        .interact()
    {
        Ok(v) => v,
        Err(_) => return Ok(None), // non-TTY, skip template
    };

    if !use_tpl {
        return Ok(None);
    }

    let items: Vec<String> = tpls
        .iter()
        .map(|t| format!("{:<14} {}", t.id, t.desc))
        .collect();

    let sel = Select::new()
        .with_prompt("Select a template")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(Some(tpls[sel].id.to_string()))
}

pub fn doctor() {
    println!("─── System ───");
    for tool in &["nix", "git", "gcc", "direnv"] {
        let found = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {tool}"))
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        let mark = if found { "✓" } else { "✗" };
        println!("  {mark} {tool}");
    }

    println!();
    println!("─── Languages ({}) ───", LANGUAGES.len());
    for l in LANGUAGES {
        println!("  {:<14} {} — {}", l.id, l.label, l.desc);
    }

    let c_tpls: Vec<&Template> = TEMPLATES.iter().filter(|t| t.lang == "c").collect();
    if !c_tpls.is_empty() {
        println!();
        println!("─── Templates for C ({}) ───", c_tpls.len());
        for t in &c_tpls {
            println!("  forge new <name> --lang c --template {:<9}  {} — {}", t.id, t.label, t.desc);
        }
    }
}
