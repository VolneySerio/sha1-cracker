use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    sync::mpsc,
};

use sha1::Digest;

pub const SHA1_HEX_LENGTH: usize = 40;

#[derive(Debug)]
pub enum Message {
    Progress { checked: usize, total: usize },
    Found(String),
    Log(String),
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Focus {
    Wordlist,
    Hash,
    StartButton,
}

pub struct App {
    pub wordlist: String,
    pub hash: String,
    pub focus: Focus,
    pub logs: Vec<String>,
    pub running: bool,
    pub checked: usize,
    pub total: usize,
    pub found: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            wordlist: String::from("wordlist.txt"),
            hash: String::new(),
            focus: Focus::Wordlist,
            logs: Vec::new(),
            running: false,
            checked: 0,
            total: 0,
            found: None,
        }
    }

    pub fn push_log(&mut self, s: impl Into<String>) {
        let s = s.into();
        self.logs.push(s);
        // keep logs bounded
        if self.logs.len() > 200 {
            self.logs.drain(0..50);
        }
    }
}

pub fn worker(
    wordlist_path: String,
    hash_to_crack: String,
    tx: mpsc::Sender<Message>,
) -> Result<(), Box<dyn Error>> {
    
    let total = BufReader::new(File::open(&wordlist_path)?).lines().count();
    let _ = tx.send(Message::Log(format!("Wordlist has {} lines", total)));
    let _ = tx.send(Message::Progress { checked: 0, total });

    
    let reader = BufReader::new(File::open(&wordlist_path)?);

    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                let _ = tx.send(Message::Log(format!("Error reading line {}: {}", i + 1, e)));
                continue;
            }
        };
        let candidate = line.trim().to_string();
        if candidate.is_empty() {
            let _ = tx.send(Message::Progress { checked: i + 1, total });
            continue;
        }

        let digest = sha1::Sha1::digest(candidate.as_bytes());
        let hex = hex::encode(digest);

        if hex == hash_to_crack {
            let _ = tx.send(Message::Found(candidate));
            let _ = tx.send(Message::Done);
            return Ok(());
        }

        if (i + 1) % 1000 == 0 || i == total.saturating_sub(1) {
            let _ = tx.send(Message::Progress { checked: i + 1, total });
            let _ = tx.send(Message::Log(format!("Checked {} / {}", i + 1, total)));
        } else {
            let _ = tx.send(Message::Progress { checked: i + 1, total });
        }
    }

    let _ = tx.send(Message::Log("No password found in wordlist.".to_string()));
    let _ = tx.send(Message::Done);
    Ok(())
}
