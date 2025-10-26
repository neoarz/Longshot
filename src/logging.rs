use crate::cache::Location;
use crate::discord::Profile;
use colored::*;
use log::{Level, SetLoggerError};
use std::io::{stdin, stdout, Read, Write};
use std::time::Instant;

#[macro_export]
macro_rules! log_error_and_exit {
    ($($arg:tt)+) => (
        error!("{}", format!($($arg)+));
        $crate::logging::pause_exit();
    )
}

#[macro_export]
macro_rules! pretty_info {
    (log: $log:expr, $($arg:tt)+) => (
        let text = format!($($arg)+);
        $log.add_message(log::Level::Info, text, false);
    );
    ($($arg:tt)+) => (
        info!("{}", format!($($arg)+));
    )
}

#[macro_export]
macro_rules! pretty_warn {
    (log: $log:expr, $($arg:tt)+) => (
        let text = format!($($arg)+);
        $log.add_message(log::Level::Warn, text, false);
    );
    ($($arg:tt)+) => (
        warn!("{}", format!($($arg)+));
    )
}

#[macro_export]
macro_rules! pretty_error {
    (log: $log:expr, $($arg:tt)+) => (
        let text = format!($($arg)+);
        $log.add_message(log::Level::Error, text, false);
    );
    ($($arg:tt)+) => (
        error!("{}", format!($($arg)+));
    )
}

#[macro_export]
macro_rules! pretty_success {
    (log: $log:expr, $($arg:tt)+) => (
        let text = format!($($arg)+);
        $log.add_message(log::Level::Info, text, true);
    );
    ($($arg:tt)+) => (
        info!("{}", format!("{}", $($arg.green())+));
    )
}

pub struct LogBlock<'a> {
    messages: Vec<LogMessage>,
    start: Instant,
    profile: &'a Profile,
    elapsed: Option<u128>,
}

impl<'a> LogBlock<'a> {
    pub fn new(profile: &'a Profile) -> Self {
        LogBlock {
            messages: Vec::new(),
            start: Instant::now(),
            profile,
            elapsed: None,
        }
    }

    pub fn add_message(&mut self, level: Level, text: String, is_success: bool) {
        let message = LogMessage {
            text,
            level,
            is_success,
        };
        self.messages.push(message);
    }

    pub fn freeze_time(&mut self) {
        self.elapsed = Some(self.start.elapsed().as_millis());
    }

    pub fn send(&mut self, location_cache: Result<Location, ()>, sender: String) {
        if self.elapsed.is_none() {
            self.freeze_time();
        }

        let location = if let Ok(location) = location_cache {
            location
        } else {
            pretty_error!(log: self, "Failed requesting location for event.");
            Location::default()
        };

        println!(
            "\n{} › ({}) [{} > {}]",
            chrono::Local::now().format("%H:%M:%S"),
            self.profile,
            location,
            sender
        );

        for message in &self.messages {
            message.send();
        }

        println!("Finished in: {}ms", self.elapsed.unwrap());
    }
}

pub struct LogMessage {
    text: String,
    level: Level,
    is_success: bool,
}

impl LogMessage {
    pub fn send(&self) {
        let text = if self.is_success {
            self.text.as_str().bright_green()
        } else {
            self.text.as_str().normal()
        };

        println!(" ({}) {}", map_level(self.level), text);
    }
}

fn map_level(level: Level) -> ColoredString {
    match level {
        Level::Info => "+".cyan(),
        Level::Trace => "-".blue(),
        Level::Error => "✗".red(),
        Level::Debug => "*".bright_black(),
        Level::Warn => "!".yellow(),
    }
}

pub fn set_up_logger() -> Result<(), SetLoggerError> {
    #[cfg(windows)]
    {
        let _ = colored::control::set_virtual_terminal(true);
    }

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}› ({}) {}",
                chrono::Local::now().format("%H:%M:%S"),
                map_level(record.level()),
                message
            ))
        })
        .level_for("serenity", log::LevelFilter::Off)
        .level_for("tracing::span", log::LevelFilter::Off)
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    println!(
        "{}\n",
        r".____                               .__            __   
|    |    ____   ____    ____  _____|  |__   _____/  |_ 
|    |   /  _ \ /    \  / ___\/  ___/  |  \ /  _ \   __\
|    |__(  <_> )   |  \/ /_/  >___ \|   Y  (  <_> )  |  
|_______ \____/|___|  /\___  /____  >___|  /\____/|__|  
        \/          \//_____/     \/     \/             ".bright_blue()
    );

    Ok(())
}

pub fn pause_exit() {
    let mut stdout = stdout();
    stdout.write_all(b"Press the enter key to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
    std::process::exit(1);
}
