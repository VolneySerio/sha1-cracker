mod app;
mod ui;

use std::{
    error::Error,
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};

use crate::app::{App, Message, worker, SHA1_HEX_LENGTH};

fn main() -> Result<(), Box<dyn Error>> {
    
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    
    let (tx, rx) = mpsc::channel::<Message>();

    let mut app = App::new();

    
    let tick_rate = Duration::from_millis(200);

    
    let res = run_app(&mut terminal, &mut app, tx, rx, tick_rate);

    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        
        while let Ok(msg) = receiver.try_recv() {
            match msg {
                Message::Progress { checked, total } => {
                    app.checked = checked;
                    app.total = total;
                }
                Message::Found(pass) => {
                    app.found = Some(pass.clone());
                    app.running = false;
                    app.push_log(format!("Password found: {}", pass));
                }
                Message::Log(s) => {
                    app.push_log(s);
                }
                Message::Done => {
                    app.running = false;
                    app.push_log("Done.".to_string());
                }
            }
        }

        
        if crossterm::event::poll(tick_rate)? {
            if let CEvent::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        
                        return Ok(());
                    }
                    KeyCode::Tab => {
                        
                        app.focus = match app.focus {
                            crate::app::Focus::Wordlist => crate::app::Focus::Hash,
                            crate::app::Focus::Hash => crate::app::Focus::StartButton,
                            crate::app::Focus::StartButton => crate::app::Focus::Wordlist,
                        };
                    }
                    KeyCode::Enter => {
                        if app.focus == crate::app::Focus::StartButton && !app.running {
                            let wordlist = app.wordlist.clone();
                            let hash = app.hash.trim().to_lowercase();
                            let tx = sender.clone();
                            if hash.len() != SHA1_HEX_LENGTH {
                                app.push_log(format!("Hash must be {} hex chars", SHA1_HEX_LENGTH));
                            } else {
                                app.push_log("Starting...".to_string());
                                app.running = true;
                                app.checked = 0;
                                app.total = 0;
                                app.found = None;
                                thread::spawn(move || {
                                    if let Err(e) = worker(wordlist, hash, tx.clone()) {
                                        let _ = tx.send(Message::Log(format!("Worker error: {}", e)));
                                        let _ = tx.send(Message::Done);
                                    }
                                });
                            }
                        } else if app.focus == crate::app::Focus::Hash {
                            app.focus = crate::app::Focus::StartButton;
                        } else if app.focus == crate::app::Focus::Wordlist {
                            app.focus = crate::app::Focus::Hash;
                        }
                    }
                    KeyCode::Backspace => {
                        if !app.running {
                            match app.focus {
                                crate::app::Focus::Wordlist => {
                                    app.wordlist.pop();
                                }
                                crate::app::Focus::Hash => {
                                    app.hash.pop();
                                }
                                crate::app::Focus::StartButton => {}
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        if !app.running {
                            match app.focus {
                                crate::app::Focus::Wordlist => app.wordlist.push(c),
                                crate::app::Focus::Hash => {
                                    if c.is_ascii_hexdigit() {
                                        app.hash.push(c.to_ascii_lowercase());
                                    }
                                }
                                crate::app::Focus::StartButton => {}
                            }
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
