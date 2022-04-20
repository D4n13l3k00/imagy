use std::{io::Write, time::Duration};

use image::GenericImageView;

use crossterm::{
    event::Event,
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

fn main() {
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        crossterm::terminal::SetTitle("imagy - in-terminal image viewer")
    )
    .unwrap();

    if std::env::args().count() < 2 {
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("Usage: "),
            SetForegroundColor(Color::Yellow),
            Print("imagy <image> [width scale [2.5]]\n"),
            ResetColor
        )
        .unwrap();
        return;
    };
    let image_file = std::env::args().nth(1).unwrap();
    if !std::path::Path::new(&image_file).exists() {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("Error: "),
            SetForegroundColor(Color::Yellow),
            Print("image file does not exist\n"),
            ResetColor
        )
        .unwrap();
        return;
    };

    let width_scale = match std::env::args().nth(2) {
        Some(arg) => arg.parse::<f64>().unwrap_or(2.5),
        None => 2.5,
    };
    let img = image::open(&image_file).unwrap();

    execute!(stdout, crossterm::cursor::Hide).unwrap();
    let (mut terminal_width, mut terminal_height) = (0, 0);
    let (img_width, img_height) = img.dimensions();
    loop {
        let (t_width, t_height) = crossterm::terminal::size().unwrap();
        if t_width != terminal_width || t_height != terminal_height {
            terminal_width = t_width;
            terminal_height = t_height;

            let mut printing_img = img.clone();

            printing_img = printing_img.resize_exact(
                (img_width as f64 * width_scale) as u32,
                img_height as u32,
                image::imageops::FilterType::Nearest,
            );

            printing_img = printing_img.thumbnail(terminal_width as u32, terminal_height as u32);

            queue!(
                stdout,
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::cursor::MoveTo(0, 0)
            )
            .unwrap();

            for (x, y, pixel) in printing_img.pixels() {
                let r = pixel.0[0];
                let g = pixel.0[1];
                let b = pixel.0[2];
                queue!(
                    stdout,
                    crossterm::cursor::MoveTo(x as u16, y as u16),
                    SetBackgroundColor(Color::Rgb {
                        r: r as u8,
                        g: g as u8,
                        b: b as u8,
                    }),
                    Print(" "),
                    ResetColor
                )
                .unwrap();
            }
        }

        stdout.flush().unwrap();

        if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(event) = crossterm::event::read().unwrap() {
                if event.code == crossterm::event::KeyCode::Char('q')
                    || event.code == crossterm::event::KeyCode::Esc
                {
                    break;
                } else if event.code == crossterm::event::KeyCode::Char('r') {
                    terminal_width = 0;
                    terminal_height = 0;
                    continue;
                } else if event.code == crossterm::event::KeyCode::Char('i') {
                    execute!(
                        stdout,
                        crossterm::cursor::MoveTo(0, 0),
                        Print(format!(
                            "Image path: {}\nImage size: {}x{}\nTerminal size: {}x{}",
                            image_file, img_width, img_height, terminal_width, terminal_height
                        ))
                    )
                    .unwrap();
                }
            }
        }
    }
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        crossterm::cursor::Show
    )
    .unwrap();
}
