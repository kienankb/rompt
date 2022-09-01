use std::io;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let username = &args[1];
    let cwd = &args[2];
    print!("{}  {}  ", username, cwd);
    io::stdout().flush().unwrap();
}
