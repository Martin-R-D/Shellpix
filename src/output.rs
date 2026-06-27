use std::io::{self, Write};

use crossterm::{
    QueueableCommand,
    style::{self, Color as CtColor, SetBackgroundColor, SetForegroundColor},
};

use crate::color::rgb_to_ansi256;
use crate::renderer::Color;

#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    TrueColor,
    Ansi256,
}

fn to_crossterm_color(c: Color, mode: ColorMode) -> CtColor {
    match mode {
        ColorMode::TrueColor => CtColor::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        },
        ColorMode::Ansi256 => CtColor::AnsiValue(rgb_to_ansi256(c.r, c.g, c.b)),
    }
}

pub fn print_to_terminal(grid: &Vec<Vec<(Color, Color, char)>>, mode: ColorMode) {
    let mut stdout = io::stdout();

    for row in grid {
        for &(fg, bg, ch) in row {
            stdout
                .queue(SetForegroundColor(to_crossterm_color(fg, mode)))
                .unwrap();
            stdout
                .queue(SetBackgroundColor(to_crossterm_color(bg, mode)))
                .unwrap();
            stdout.queue(style::Print(ch)).unwrap();
        }
        stdout.queue(style::ResetColor).unwrap();
        stdout.queue(style::Print('\n')).unwrap();
        stdout.flush().unwrap();
    }

    stdout.queue(style::ResetColor).unwrap();
    stdout.flush().unwrap();
}
