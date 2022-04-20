use crossterm::event::KeyCode;
use crossterm::{
    cursor,
    event::{self, Event},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use ctrlc;
use image::GenericImageView;
use std::{io::Write, time::Duration};

fn ctrlc_handler() {
    ctrlc::set_handler(|| {
        let mut stduot = std::io::stdout();
        execute!(
            stduot,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(1, 1),
            cursor::Show,
            ResetColor
        )
        .unwrap();
        std::process::exit(0);
    })
    .unwrap();
}

fn main() {
    let mut stdout = std::io::stdout();
    ctrlc_handler();
    execute!(
        stdout,
        terminal::SetTitle("imagy - in-terminal image viewer")
    )
    .unwrap();
    let __width_scale = if cfg!(windows) { 2.5 } else { 2.0 };

    // parse args
    if std::env::args().count() < 2 {
        execute!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("Usage: "),
            SetForegroundColor(Color::Yellow),
            Print(format!("imagy <image> [width scale [{}]]\n", __width_scale)),
            ResetColor
        )
        .unwrap();
        return;
    };
    let image_file = std::env::args().nth(1).unwrap();
    if !std::path::Path::new(&image_file).is_file() {
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

    // getting scale
    let width_scale = match std::env::args().nth(2) {
        Some(arg) => arg.parse::<f64>().unwrap_or(__width_scale),
        None => __width_scale,
    };

    // getting image
    let img = image::open(&image_file);
    if img.is_err() {
        execute!(
            stdout,
            SetForegroundColor(Color::Red),
            Print("Error: "),
            SetForegroundColor(Color::Yellow),
            Print(format!("{}\n", img.unwrap_err().to_string())),
            ResetColor
        )
        .unwrap();
        return;
    };
    let img = img.unwrap();

    execute!(stdout, cursor::Hide).unwrap();
    let (mut terminal_width, mut terminal_height) = (0, 0);
    let (img_width, img_height) = img.dimensions();
    loop {
        // actual terminal size
        let (t_width, t_height) = terminal::size().unwrap();

        // redraw only if terminal size changed
        if t_width != terminal_width || t_height != terminal_height {
            terminal_width = t_width;
            terminal_height = t_height;

            let mut printing_img = img.clone();

            // resize image to fit in terminal
            printing_img = printing_img
                .resize_exact(
                    (img_width as f64 * width_scale) as u32,
                    img_height as u32,
                    image::imageops::FilterType::Nearest,
                )
                .thumbnail(terminal_width as u32, terminal_height as u32);

            // clear terminal
            queue!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            )
            .unwrap();

            // draw pixel by pixel
            for (x, y, pixel) in printing_img.pixels() {
                queue!(
                    stdout,
                    cursor::MoveTo(x as u16, y as u16),
                    SetBackgroundColor(Color::Rgb {
                        r: pixel.0[0] as u8,
                        g: pixel.0[1] as u8,
                        b: pixel.0[2] as u8,
                    }),
                    Print(" "),
                    ResetColor
                )
                .unwrap();
            }
        }
        // flush for printing
        stdout.flush().unwrap();

        // handle keypress
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char('r') => {
                        terminal_width = 0;
                        terminal_height = 0;
                        continue;
                    }
                    KeyCode::Char('i') => {
                        execute!(
                            stdout,
                            cursor::MoveTo(0, 0),
                            Print(format!(
                                "Image path: {}\nImage size: {}x{}\nTerminal size: {}x{}",
                                image_file, img_width, img_height, terminal_width, terminal_height
                            ))
                        )
                        .unwrap();
                    }
                    _ => {}
                }
            }
        }
    }

    // clear terminal on exit
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    )
    .unwrap();
}
