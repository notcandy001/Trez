use anyhow::Result;
use color_engine::{Engine, EngineOptions};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: color-engine <path-to-image>");
        std::process::exit(1);
    }

    let path = &args[1];
    println!("Analyzing image: {}", path);

    let engine = Engine::new(EngineOptions::default());
    let theme = engine.extract_theme_from_path(path)?;

    let json = serde_json::to_string_pretty(&theme)?;
    println!("{}", json);

    Ok(())
}
