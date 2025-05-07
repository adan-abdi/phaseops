pub fn info(message: &str) {
    println!("{}", message);
}

#[allow(dead_code)]
pub fn error(message: &str) {
    eprintln!("{}", message);
}

#[allow(dead_code)]
pub fn usage() {
    println!("Usage: phaseops <cli|web_ui|hud|cache-init|cache-commit> [options]");
}
