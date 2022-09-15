use std::io;
use std::io::Write;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let username = &args[1];
    let cwd = env::current_dir().unwrap();
    let cwd_str = cwd.to_str().expect("failed to get current working directory").split("/");
    print!("{}  ", username);
    for path_frag in cwd_str {
        print!("{}  ", path_frag);
    }
    io::stdout().flush().unwrap();
}
