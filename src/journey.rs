#![allow(missing_docs)] 

use chrono::prelude::*;
use failure::*;
use structopt::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn start() -> Result<(), Error> {
    let opts = Opts::from_args();

    match opts {
        Opts::New(new) => match new {
            New::Entry => new_entry()?,
        },
    }

    Ok(())
}

fn new_entry() -> Result<(), Error> {
    let mut rl = Editor::<()>::new();
    
    println!("Date: (yyyy-mm-dd)");
    let date_string = rl
        .readline(">>> ")?
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    
    let date = Utc.ymd(
        date_string[0].parse::<i32>()?,
        date_string[1].parse::<u32>()?,
        date_string[2].parse::<u32>()?,
    );

    println!("\nTag (empty for no tag)");
    let tag_name = rl.readline(">>> ")?;
    
    println!("\nEntry: (ctrl-d) to finish");
    let mut info = String::new();

    loop {
        match rl.readline(">>> ") {
            Ok(line) => {
                info.push_str(&line);
                info.push('\n');
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            e => {
                let _ = e?;
            }
        }
    }

    let entry = Entry {
        date,
        tag: Tag{name: tag_name},
        info: info.trim().to_string(),
    };

    entry.print()?;
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "jny", about = "journey is a command line tool to keep track of your journey and to help you get organized.")]
enum Opts {
    #[structopt(name = "new")]
    New(New),
}

#[derive(StructOpt)]
enum New {
    #[structopt(name = "entry")]
    Entry,
}

#[derive(Debug)]
struct Entry {
    tag: Tag,
    date: Date<Utc>,
    info: String,
}

impl Entry {

    fn print(&self) -> Result<(), Error> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut stdout, "Date: ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "{}", self.date)?;
        stdout.reset()?;
        writeln!(&mut stdout)?;
        writeln!(&mut stdout, "{}", self.info)?;
        writeln!(&mut stdout)?;
        Ok(())
    }

}

#[derive(Debug)]
struct Tag {
    name: String,
}
