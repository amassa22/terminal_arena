use std::io::{stdout, Write};
use std::time::Duration;
use std::{io, thread};

use clap::Error;
use console::Term;
use crossterm::cursor::MoveToNextLine;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::QueueableCommand;

pub fn slow_type(text: &str) {
    let mut stdout = io::stdout();
    let mut chars_printed = 0;
    // Enable raw mode
    if let Ok(_) = enable_raw_mode() {
        for c in text.chars() {
            // Check for space bar press to print the rest of the string immediately
            if event::poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if key_event.code == KeyCode::Char(' ') {
                        // print!("{}", &text[text.chars().count()..]);
                        print!("{}", &text[chars_printed..]);
                        break;
                    }
                }
            }

            // Print each character
            print!("{}", c);
            stdout.flush().unwrap();
            chars_printed += c.len_utf8();
            thread::sleep(Duration::from_millis(25));
        }

        // Move to the next line after finishing printing
        stdout.queue(MoveToNextLine(1)).unwrap();
        stdout.flush().unwrap();

        // Disable raw mode
        disable_raw_mode().unwrap();
    } else {
        println!("{}", text); // Fallback if raw mode can't be enabled
    }
}

pub fn print_line() {
    let (cols, _) = size().unwrap(); // Get the size of the terminal (columns, rows)
    println!("{}", "═".repeat(cols as usize)); // Print a line that matches the width of the terminal
}

pub fn clear_screen() {
    let term = Term::stdout();
    println!("Press any key to continue...");
    term.read_key().unwrap();
    term.clear_screen().unwrap();
}

pub fn print_logo() {
    let art = r#" _____                   _             _      _                         
|_   _|__ _ __ _ __ ___ (_)_ __   __ _| |    / \   _ __ ___ _ __   __ _ 
  | |/ _ \ '__| '_ ` _ \| | '_ \ / _` | |   / _ \ | '__/ _ \ '_ \ / _` |
  | |  __/ |  | | | | | | | | | | (_| | |  / ___ \| | |  __/ | | | (_| |
  |_|\___|_|  |_| |_| |_|_|_| |_|\__,_|_| /_/   \_\_|  \___|_| |_|\__,_|"#;

    print_centered_art(art).unwrap();
}

fn print_centered_art(art: &str) -> Result<(), Error> {
    let (terminal_width, _) = size()?;
    let stdout = stdout();
    let mut handle = stdout.lock();

    for line in art.lines() {
        let line_length = line.chars().count();
        let padding = (terminal_width as usize).saturating_sub(line_length) / 2;
        let padded_line = format!("{:padding$}{}", "", line, padding = padding);
        writeln!(handle, "{}", padded_line)?;
    }

    Ok(())
}
