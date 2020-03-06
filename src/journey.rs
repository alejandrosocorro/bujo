#![allow(missing_docs)]

use chrono::prelude::*;
use structopt::*;
use failure::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(StructOpt)]
#[structopt(name = "jny", about = "journey is a command line tool to keep track of your journey and to help you get organized.")]
enum Opts {
    #[structopt(name = "new")]
    New(New),
}

#[derive(StructOpt)]
enum New {
    #[structopt(name = "bullet")]
    Bullet,
}

pub fn start() -> Result<(), Error> {
let opts = Opts::from_args();

    match opts {
        Opts::New(new) => match new {
            New::Bullet => new_bullet()?,
        },
    }

    Ok(())
}

fn new_bullet() -> Result<(), Error> {
    let mut rl = Editor::<()>::new();

    println!("\nPress (ctrl-d) to finish");
    let mut info = String::new();

    loop {
        match rl.readline(">>>  ") {
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

    let bullet = Bullet {
        date: Utc::now().date(),
        tag: Tag::default(),
        category: Category::Task,
        value: info.trim().to_string(),
    };

    bullet.print()?;
    Ok(())
}

#[derive(Debug)]
enum Category {
    Task,
    Note,
    Event,
}

#[derive(Debug, Default)]
struct Tag {
    name: String,
}

#[derive(Debug)]
struct Bullet {
    tag: Tag,
    date: Date<Utc>,
    category: Category,
    value: String,
}

impl Bullet {
    fn print(&self) -> Result<(), Error> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut stdout, "Date: ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut stdout, "{}", self.date)?;
        stdout.reset()?;
        writeln!(&mut stdout)?;
        writeln!(&mut stdout, "{}", self.value)?;
        writeln!(&mut stdout)?;
        Ok(())
    }
}
