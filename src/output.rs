use std::io::{BufWriter, Write};
use std::sync::mpsc::Receiver;

use colored::Colorize;

use crate::deserialize::AppInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Info {
        appname: String,
        bucket_name: String,
        details: AppInfo,
    },
    Close,
}

pub fn worker(query: String, receiver: Receiver<Message>) {
    let stdout = std::io::stdout();
    let mut buf_stdout = BufWriter::new(stdout.lock());

    while let Ok(message) = receiver.recv() {
        if let Message::Info {
            appname,
            bucket_name,
            details: detail,
        } = message
        {
            // U+2500	─
            // U+2502	│
            // U+250C	┌
            // U+2510	┐
            // U+2514	└
            // U+2518	┘

            let backcolor = |s: &str| {
                if let Some(i) = s.to_lowercase().find(&query) {
                    let j = i + query.chars().count();
                    format!(
                        "{}{}{}",
                        &s[..i],
                        s[i..j].bold().on_yellow().black(),
                        &s[j..]
                    )
                } else {
                    s.to_string()
                }
            };

            let width = crossterm::terminal::size().unwrap().0 as usize;
            writeln!(&mut buf_stdout, "┌{}┐", "─".repeat(width - 2)).unwrap();
            writeln!(
                &mut buf_stdout,
                "  {} in {}",
                backcolor(&appname),
                bucket_name.blue()
            )
            .unwrap();
            writeln!(
                &mut buf_stdout,
                "  description: {}",
                backcolor(&detail.description)
            )
            .unwrap();
            writeln!(&mut buf_stdout, "  version: {}", detail.version.cyan()).unwrap();
            writeln!(&mut buf_stdout, "  homepage: {}", detail.homepage.purple()).unwrap();
            writeln!(&mut buf_stdout, "  license: {}", detail.license.green()).unwrap();
            if let Some(notes) = detail.notes {
                writeln!(&mut buf_stdout, "  notes: {}", notes).unwrap();
            }
            writeln!(&mut buf_stdout, "└{}┘", "─".repeat(width - 2)).unwrap();
        } else {
            break;
        }
    }

    buf_stdout.flush().unwrap();
}
