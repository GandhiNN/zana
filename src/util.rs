use chrono::{DateTime, NaiveDateTime, Utc};
use csv::Writer;
use std::error::Error;
use std::fmt::Debug;
use std::{env, io};
use tabled::settings::Style;
use tabled::{Table, Tabled};

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn pause() -> Result<(), std::io::Error> {
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line
    // so we print without a newline
    write!(stdout, "Press any key to continue...")?;
    stdout.flush()?;

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8])?;

    Ok(())
}

pub fn read_from_stdin() -> Result<String, std::io::Error> {
    use std::io::prelude::*;

    let mut buffer = String::new();
    let mut stdout = io::stdout();

    write!(stdout, "Select URL key or type exit to quit program : ")?;
    stdout.flush()?;

    io::stdin().read_line(&mut buffer)?; // includes '\n'
    Ok(buffer)
}

pub fn print_environment_variables() {
    for (k, v) in env::vars() {
        println!("{}, {}", k, v);
    }
}

pub fn pretty_print<T: Tabled + Debug>(iterables: Vec<T>) {
    let mut table = Table::new(&iterables);
    table.with(Style::psql());
    println!("{}", table);
}

pub fn write_csv<T: serde::Serialize>(iterables: Vec<T>) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_writer(vec![]);
    for row in &iterables {
        writer.serialize(row)?
    }
    let data = String::from_utf8(writer.into_inner()?)?;
    println!("{}", data);
    Ok(())
}

pub fn convert_naive_datetime_to_utc(date: &str) -> DateTime<Utc> {
    let naive_datetime = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
    DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc)
}
