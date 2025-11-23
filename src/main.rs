use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::{Modifier, Style},
    text::Text,
    widgets::{
        Block, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
};

mod deserialize;
mod search;

fn main() -> anyhow::Result<()> {
    let apps = search::search()?;
    let content_length = apps.len();

    let mut terminal = ratatui::init();
    let mut timer = std::time::Instant::now();

    let mut scroll_offet = 0usize;
    let mut scrollbar_state = ScrollbarState::new(content_length);

    'tui: loop {
        terminal.draw(|f| {
            let paras = apps
                .iter()
                .map(|app_info| {
                    let txt = Text::from_iter([
                        format!("{} in {}", app_info.name, app_info.bucket),
                        app_info.version.clone(),
                        app_info.description.clone(),
                        app_info.homepage.clone(),
                        app_info.license.clone(),
                        app_info.notes.clone(),
                    ]);
                    Paragraph::new(txt).block(Block::bordered())
                })
                .collect::<Vec<_>>();

            f.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                f.area(),
                &mut scrollbar_state,
            );
        })?;

        if event::poll(timer.elapsed())? {
            timer = std::time::Instant::now();

            if let Ok(evt) = event::read()
                && let Event::Key(key) = evt
            {
                match key.code {
                    KeyCode::Char('q') => break 'tui,

                    KeyCode::Down => {
                        scroll_offet = (scroll_offet + 1).min(content_length);
                        scrollbar_state = scrollbar_state.position(scroll_offet);
                    }
                    KeyCode::Up => {
                        scroll_offet = scroll_offet.saturating_sub(1);
                        scrollbar_state = scrollbar_state.position(scroll_offet);
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}
