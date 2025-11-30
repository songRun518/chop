use std::path::PathBuf;

use clap::Parser;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget, Wrap},
};

use crate::{error::MyError, search::AppInfo};

mod config;
mod error;
mod manifest;
mod search;

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct ArgParser {
    query: String,

    #[arg(short = 'p', long)]
    root_path: Option<PathBuf>,
}

fn main() -> Result<(), MyError> {
    let args = ArgParser::parse();
    let apps = search::search(&args)?;

    if apps.is_empty() {
        println!("No matches found");
        return Ok(());
    }

    let mut terminal = ratatui::init();

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    'tui: loop {
        terminal.draw(|f| {
            let items = apps
                .iter()
                .map(|app| ListItem::new(format!("{}/{}", app.bucket, app.name,)))
                .collect::<Vec<_>>();
            let list = List::new(items).highlight_symbol(">> ").highlight_style(
                Style::default()
                    .fg(ratatui::style::Color::Blue)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            );

            let appinfo = &apps[list_state.selected().unwrap_or(0)];

            let layout = Layout::new(
                ratatui::layout::Direction::Horizontal,
                [Constraint::Fill(1), Constraint::Fill(2)],
            )
            .split(f.area());

            f.render_stateful_widget(list, layout[0], &mut list_state);
            render_appinfo(&args.query, appinfo, f, layout[1]);
        })?;

        if let Ok(evt) = event::read()
            && let Event::Key(key) = evt
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break 'tui,

                KeyCode::Up => {
                    list_state.select(Some(
                        list_state.selected().unwrap_or(0).saturating_sub(1),
                    ));
                }
                KeyCode::Down => {
                    list_state.select(Some(
                        (list_state.selected().unwrap_or(0) + 1).min(apps.len() - 1),
                    ));
                }
                _ => {}
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn render_appinfo(
    query: &str,
    info: &AppInfo,
    f: &mut ratatui::Frame,
    area: ratatui::prelude::Rect,
) {
    let name_l = if let Some(i) = info.name.to_lowercase().find(query) {
        let j = i + query.chars().count();
        Line::from_iter([
            Span::from(format!("{}/", info.bucket)),
            Span::from(&info.name[..i]),
            Span::from(&info.name[i..j]).yellow().bold(),
            Span::from(&info.name[j..]),
            Span::from(format!("  {}", info.version)).cyan(),
        ])
    } else {
        Line::from_iter([
            Span::from(format!("{}/", info.bucket)),
            Span::from(&info.name),
            Span::from(format!("  {}", info.version)).cyan(),
        ])
    };

    let description_l = if let Some(i) = info.description.to_lowercase().find(query) {
        let j = i + query.chars().count();
        Line::from_iter([
            Span::from("    "),
            Span::from(&info.description[..i]),
            Span::from(&info.description[i..j]).yellow().bold(),
            Span::from(&info.description[j..]),
        ])
    } else {
        Line::from_iter([Span::from("    "), Span::from(&info.description)])
    };

    let homepage_l = Line::from_iter([
        Span::from("\u{1F310}  "),
        Span::from(&info.homepage).magenta(),
    ]);

    let license_l =
        Line::from_iter([Span::from("\u{1F4DC}  "), Span::from(&info.license).green()]);

    let notes_l = if let Some(notes) = &info.notes {
        Line::from_iter([Span::from("\u{1F4DA}  "), Span::from(notes)])
    } else {
        Line::default()
    };

    let text = Text::from_iter([name_l, description_l, homepage_l, license_l, notes_l]);
    let para = Paragraph::new(text)
        .block(Block::bordered())
        .wrap(Wrap { trim: false });

    f.render_widget(para, area);
}
