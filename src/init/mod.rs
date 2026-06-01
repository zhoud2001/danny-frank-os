use crate::shell::Shell;

pub fn start() {
    println!("Starting init process...");

    let mut shell = Shell::new();
    shell.run();
}
