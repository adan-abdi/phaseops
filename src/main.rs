mod cli;
mod logger;
mod git_backend;
mod web_ui;
mod hud;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("cli") => cli::run(),
        Some("web_ui") => web_ui::run(),
        Some("git_backend") => git_backend::run(),
        Some("hud") => hud::run(),
        _ => {
            eprintln!("Usage: phaseops <cli|web_ui|hud> [options]");
        }
    }
}