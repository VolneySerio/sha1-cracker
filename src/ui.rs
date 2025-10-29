use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;


pub fn ui(f: &mut Frame<'_>, app: &App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(6),
            ]
            .as_ref(),
        )
        .split(size);

    
    let wordlist_block = Block::default().borders(Borders::ALL).title("Wordlist");
    let wordlist_para = Paragraph::new(app.wordlist.as_str())
        .block(wordlist_block)
        .wrap(Wrap { trim: true });
    f.render_widget(wordlist_para, chunks[0]);

    let hash_block = Block::default().borders(Borders::ALL).title("SHA1 hash (hex)");
    let mut hash_text = app.hash.clone();
    if app.focus == crate::app::Focus::Hash && !app.running {
        hash_text.push('_');
    }
    let hash_para = Paragraph::new(hash_text.as_str()).block(hash_block).wrap(Wrap { trim: true });
    f.render_widget(hash_para, chunks[1]);

    
    let status = if app.running {
        format!("Running — checked {}/{}", app.checked, app.total)
    } else if let Some(found) = &app.found {
        format!("FOUND: {}", found)
    } else {
        "Idle".to_string()
    };

    let start_label = if app.focus == crate::app::Focus::StartButton {
        
        format!("[ Start ]    {}", status)
    } else {
        format!("[ Start ]    {}", status)
    };

    let status_block = Block::default().borders(Borders::ALL).title("Control");
    let status_para = Paragraph::new(start_label.as_str()).block(status_block);
    f.render_widget(status_para, chunks[2]);

    
    render_logs_and_progress(f, chunks[3], app);
}


fn render_logs_and_progress(f: &mut Frame<'_>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    let gauge_block = Block::default().borders(Borders::ALL).title("Progress");
    let ratio = if app.total == 0 {
        0.0
    } else {
        (app.checked as f64) / (app.total as f64)
    };
    let gauge = Gauge::default()
        .block(gauge_block)
        .gauge_style(Style::default().add_modifier(Modifier::BOLD))
        .ratio(ratio)
        .label(format!("{}/{}", app.checked, app.total));
    f.render_widget(gauge, chunks[0]);

    let logs_block = Block::default().borders(Borders::ALL).title("Logs");
    let items: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(100)
        .map(|l| ListItem::new(l.as_str()))
        .collect();
    let list = List::new(items).block(logs_block);
    f.render_widget(list, chunks[1]);
}
