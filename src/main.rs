use std::io;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cwd = &args[1];
    let home_dir = env::var("HOME").unwrap_or("none".to_string());
    let replaced_cwd = cwd.replace(&home_dir, "~");
    let split_cwd = replaced_cwd.split("/");
    for path_frag in split_cwd {
        print!("{}  ", path_frag);
    }
    io::stdout().flush().unwrap();
}
