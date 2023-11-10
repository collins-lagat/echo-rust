use clap::{Arg, ArgAction, Command};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let matches = Command::new("echo")
        .arg(Arg::new("STRING").action(ArgAction::Append))
        .get_matches();

    let values: Vec<String> = match matches.get_many::<String>("STRING") {
        Some(s) => s.map(|s| s.to_string()).collect(),
        None => vec![String::new()],
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            // write! is like format! but it will write the formatted string
            // into the given buffer (first argument)
            write!(handle, " ")?;
        }
        write!(handle, "{value}")?;
    }

    // writeln! is like write! but it will append a newline character
    // at the end of the string
    writeln!(handle)?;

    return Ok(());
}
