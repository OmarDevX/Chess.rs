use std::process::{Command, Child, Stdio};
use std::io::{BufReader, BufRead, Write};
use std::time::Duration;
use std::thread;

pub struct Stockfish {
    process: Child,
}

impl Stockfish {
    pub fn new(path: &str) -> Self {
        let process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start stockfish process");

        // Wait for Stockfish to initialize
        thread::sleep(Duration::from_millis(500));

        Self { process }
    }

    pub fn set_skill_level(&mut self, level: u8) {
        self.send_command(&format!("setoption name Skill Level value {}", level));
    }

    pub fn set_position(&mut self, fen: &str) {
        self.send_command(&format!("position fen {}", fen));
    }

    pub fn get_best_move(&mut self, time_ms: u64) -> Option<String> {
        self.send_command(&format!("go movetime {}", time_ms));
        
        let stdout = self.process.stdout.as_mut().expect("Failed to open stdout");
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            if line.starts_with("bestmove") {
                return line.split_whitespace().nth(1).map(String::from);
            }
        }
        None
    }

    fn send_command(&mut self, command: &str) {
        let stdin = self.process.stdin.as_mut().expect("Failed to open stdin");
        writeln!(stdin, "{}", command).expect("Failed to write to stdin");
    }
}

impl Drop for Stockfish {
    fn drop(&mut self) {
        self.send_command("quit");
        let _ = self.process.wait();
    }
}
