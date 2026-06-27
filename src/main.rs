use std::process;

use clap::{Parser, ValueEnum};
use crossterm::terminal;

mod image_loader;
mod output;
mod renderer;

#[derive(Debug, Clone, ValueEnum)]
enum RenderMode {
    Halfblock,
    Ascii,
    Braille,
}

#[derive(Parser, Debug)]
#[command(name = "shellpix")]
#[command(about = "Render images, GIFs, and video as colored ASCII/Unicode art directly in your terminal")]
struct Args {
    /// Image file path to render
    input: String,

    /// Override output width in columns
    #[arg(short, long)]
    width: Option<u32>,

    /// Override output height in rows
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Render mode
    #[arg(short, long, value_enum, default_value_t = RenderMode::Halfblock)]
    mode: RenderMode,
}

fn main() {
    let args = Args::parse();

    let (term_cols, term_rows) = terminal::size().unwrap_or_else(|e| {
        eprintln!("Error: failed to detect terminal size: {e}");
        process::exit(1);
    });

    let target_width = args.width.unwrap_or(term_cols as u32);
    let target_height = args.height.unwrap_or((term_rows as u32) * 2);

    let img = image_loader::load_and_resize(&args.input, target_width, target_height);
    let grid = renderer::render_halfblock(&img);
    output::print_to_terminal(&grid);
}
