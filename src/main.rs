use std::io;
use std::io::Write;

use parser::calc;

mod lexer;
mod parser;
mod test;

fn main() {
    let exit_cmd = vec!["exit", "quit", "q"];
    let input = io::stdin();
    let mut output = io::stdout();
    loop {
        output.write_all(b"calc>> ").unwrap();
        output.flush().unwrap();

        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();

        let line = &buffer.trim();
        if buffer.is_empty() || exit_cmd.contains(line) {
            break;
        }

        if line == &"whosyourdaddy" {
            print_banner();
            continue;
        }

        if line.is_empty() {
            continue;
        }
        output
            .write_all(calc(line).as_ref())
            .unwrap();
    }
}

fn print_banner() {
    print!(
        r#"
  _____      _            _       _
 / ____|    | |          | |     | |
| |     __ _| | ___ _   _| | __ _| |_ ___  _ __
| |    / _` | |/ __| | | | |/ _` | __/ _ \| '__|
| |___| (_| | | (__| |_| | | (_| | || (_) | |
 \_____\__,_|_|\___|\__,_|_|\__,_|\__\___/|_|

 Made By: LebranceBW
"#
    );
}
