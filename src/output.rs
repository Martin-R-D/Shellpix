use std::io::{self, Write};

use crossterm::{
    QueueableCommand,
    style::{self, Color as CtColor, SetBackgroundColor, SetForegroundColor},
};

use crate::renderer::Color;

pub fn print_to_terminal(grid: &Vec<Vec<(Color, Color, char)>>) {
    let mut stdout = io::stdout();

    for row in grid {
        for &(fg, bg, ch) in row {
            stdout
                .queue(SetForegroundColor(CtColor::Rgb {
                    r: fg.r,
                    g: fg.g,
                    b: fg.b,
                }))
                .unwrap();
            stdout
                .queue(SetBackgroundColor(CtColor::Rgb {
                    r: bg.r,
                    g: bg.g,
                    b: bg.b,
                }))
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
