use std::{env, io};

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn pause() {
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line
    // so we print without a newline
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn read_from_stdin() -> io::Result<String> {
    use std::io::prelude::*;

    let mut buffer = String::new();
    let mut stdout = io::stdout();

    write!(stdout, "Select URL key or type exit to quit program : ").unwrap();
    stdout.flush().unwrap();

    io::stdin().read_line(&mut buffer)?; // includes '\n'
    Ok(buffer)
}

pub fn print_environment_variables() {
    for (k, v) in env::vars() {
        println!("{}, {}", k, v);
    }
}
