use image::Frame;
use mouse_rs::{types::keys::Keys, Mouse};
use scrap::{Capturer, Display};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::ErrorKind::WouldBlock;
use std::io::Write;
use std::thread;
use std::time::Duration;
use tokio::{task, time}; // 1.3.0

extern crate scrap;

fn screencapturer() -> Vec<u8> {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

    loop {
        // Wait until there's a frame.

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        return buffer.to_owned();
    }
}

fn detect_chicken(screen_buffer: Vec<u8>) -> (usize, usize) {
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());
    let stride = screen_buffer.len() / h;
    println!("{}", stride);


    for y in 0..h {
        for x in 0..w {
            //stride is the total number of elements per column
            // 4 is just the number of elements in ARGB pixel.
            let i = stride * y + 4 * x;
            //stored in BGRA so screen_buffer[i+2] == Red,  screen_buffer[i+2] == Green and screen_buffer[i] == Blue
            if screen_buffer[i + 2] < 140
                && screen_buffer[i + 2] > 130
                && screen_buffer[i + 1] < 15
                && screen_buffer[i] < 15
            {
                return (x, y);
            }
        }
    }
    return (0,0)
}
fn get_delay_from_user() -> u64 {
    print!("input delay in seconds: ");
    io::stdout().flush().unwrap();
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text);

    let trimmed = input_text.trim();
    trimmed.parse::<u64>().unwrap()
}
fn move_mouse_and_click(mouse: &Mouse) {
    let pos = mouse.get_position().unwrap();
    let screen_buffer: Vec<u8> = screencapturer();
    let (x,y) = detect_chicken(screen_buffer);
    mouse
        .move_to((x) as i32, (y + 10) as i32)
        .expect("Unable to move mouse");
}
#[tokio::main]
async fn main() {
    let forever = task::spawn(async {
        let mouse = Mouse::new();
        let mut interval = time::interval(Duration::from_secs(get_delay_from_user()));
        loop {
            interval.tick().await;
            move_mouse_and_click(&mouse);
        }
    });
    forever.await;
}
