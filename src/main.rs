use clap::{Parser, ValueEnum};

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

    println!("Input file: {}", args.input);
    if let Some(w) = args.width {
        println!("Width: {w}");
    }
    if let Some(h) = args.height {
        println!("Height: {h}");
    }
    println!("Mode: {:?}", args.mode);
}
