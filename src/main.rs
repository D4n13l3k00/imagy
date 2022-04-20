use std::{io::Write, time::Duration};

use image::GenericImageView;

use crossterm::{
    event::Event,
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};

fn main() {
    if std::env::args().count() != 2 {
        println!("Usage: {} <image_file>", std::env::args().nth(0).unwrap());
        return;
    }
    crossterm::terminal::SetTitle("imagy - in-terminal image viewer");
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::cursor::Hide).unwrap();

    let image_file = std::env::args().nth(1).unwrap();

    let img = image::open(&image_file).unwrap();

    let (mut terminal_width, mut terminal_height) = (0, 0);
    let (img_width, img_height) = img.dimensions();
    loop {
        let (t_width, t_height) = crossterm::terminal::size().unwrap();
        if t_width != terminal_width || t_height != terminal_height {
            terminal_width = t_width;
            terminal_height = t_height;

            let mut printing_img = img.clone();
            if (terminal_width as u32) < img_width || (terminal_height as u32) < img_height {
                printing_img = printing_img.resize(
                    terminal_width as u32,
                    terminal_height as u32,
                    image::imageops::FilterType::Nearest,
                );
            }

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

                if x == 0 {
                    execute!(stdout, crossterm::cursor::MoveTo(0, y as u16)).unwrap();
                }
                queue!(
                    stdout,
                    SetBackgroundColor(Color::Rgb {
                        r: r as u8,
                        g: g as u8,
                        b: b as u8,
                    }),
                    Print("  "),
                    ResetColor
                )
                .unwrap();
            }
        }
        // flush the terminal via crossterm
        stdout.flush().unwrap();
        if crossterm::event::poll(Duration::from_millis(10)).unwrap() {
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
    // clear terminal
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    )
    .unwrap();
}
