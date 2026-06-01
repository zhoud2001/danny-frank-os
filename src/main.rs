mod init;
mod kernel;
mod shell;

fn main() {
    kernel::boot();
    init::start();
}
