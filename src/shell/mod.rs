use std::io::{self, Write};

pub struct Shell {
    is_running: bool,
}

impl Shell {
    pub fn new() -> Self {
        Self { is_running: true }
    }

    pub fn run(&mut self) {
        println!("DFOS shell started. Type 'help' for available commands.");

        while self.is_running {
            match self.read_command() {
                Ok(Some(command)) => self.handle_command(&command),
                Ok(None) => self.shutdown(),
                Err(error) => {
                    eprintln!("Shell error: {error}");
                    self.shutdown();
                }
            }
        }
    }

    fn read_command(&self) -> io::Result<Option<String>> {
        print!("dfos> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;

        if bytes_read == 0 {
            return Ok(None);
        }

        Ok(Some(input.trim().to_string()))
    }

    fn handle_command(&mut self, command: &str) {
        let mut parts = command.split_whitespace();

        match parts.next() {
            None => {}
            Some("help") => self.print_help(),
            Some("version") => println!("DFOS {}", env!("CARGO_PKG_VERSION")),
            Some("echo") => println!("{}", parts.collect::<Vec<_>>().join(" ")),
            Some("shutdown") | Some("exit") => self.shutdown(),
            Some(unknown) => println!("Unknown command: {unknown}"),
        }
    }

    fn print_help(&self) {
        println!("Available commands:");
        println!("  help       Show this help message");
        println!("  version    Show the DFOS version");
        println!("  echo       Print text back to the shell");
        println!("  shutdown   Shut down DFOS");
        println!("  exit       Alias for shutdown");
    }

    fn shutdown(&mut self) {
        println!("Shutting down DFOS...");
        self.is_running = false;
    }
}
