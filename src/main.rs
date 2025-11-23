use std::path::PathBuf;

use clap::Parser;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
};

mod deserialize;
mod search;

#[derive(Debug, serde::Deserialize, clap::Parser)]
#[command(arg_required_else_help = true)]
struct ArgParser {
    query: String,

    #[arg(short = 'p', long)]
    scoop_root_path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = ArgParser::parse();
    let apps = search::search(&args)?;

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
            let (appname0, appname1, appname2) = {
                let i = appinfo
                    .name
                    .to_lowercase()
                    .find(&args.query.to_lowercase())
                    .unwrap();
                let j = i + appinfo.name.chars().count();
                (
                    Span::from(&appinfo.name[..i]),
                    Span::from(&appinfo.name[i..j]).yellow(),
                    Span::from(&appinfo.name[j..]),
                )
            };
            let txt = Text::from(vec![
                Line::from_iter([Span::from(&appinfo.version).cyan()]),
                Line::from(format!("  {}", appinfo.description)),
                Line::from_iter([Span::from("\u{1F517}  "), Span::from(&appinfo.homepage)]),
                Line::from_iter([Span::from("\u{2696}   "), Span::from(&appinfo.license)]),
                Line::from(appinfo.notes.as_str()),
            ]);
            let para = Paragraph::new(txt)
                .block(Block::bordered())
                .wrap(Wrap { trim: false });

            let layout = Layout::new(
                ratatui::layout::Direction::Horizontal,
                [Constraint::Fill(1), Constraint::Fill(2)],
            )
            .split(f.area());

            f.render_stateful_widget(list, layout[0], &mut list_state);
            f.render_widget(para, layout[1]);
        })?;

        if let Ok(evt) = event::read()
            && let Event::Key(key) = evt
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') => break 'tui,

                KeyCode::Up => {
                    list_state.select(Some(list_state.selected().unwrap_or(0).saturating_sub(1)));
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
