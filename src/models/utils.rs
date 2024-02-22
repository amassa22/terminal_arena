use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use console::Term;
use crossterm::cursor::MoveToNextLine;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
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
    println!("{}", "═".repeat(30)); // Adjust the number of repetitions to fit your layout
}

pub fn clear_screen() {
    let term = Term::stdout();
    println!("Press any key to continue...");
    term.read_key().unwrap();
    term.clear_screen().unwrap();
}
