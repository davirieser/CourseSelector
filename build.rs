use std::process::Command;

macro_rules! debug_print {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    // Rebuild if files changed
    println!("cargo:rerun-if-changed=./static/src");
    // Compile Typescript and Webpack
    match Command::new("npm")
        .args(["run", "build"])
        .current_dir(std::fs::canonicalize("./static").unwrap())
        .spawn()
    {
        Ok(_) => {
            debug_print!("Rebuild Typescript");
        }
        Err(_) => {
            debug_print!("Error rebuilding Typescript");
        }
    }
}
