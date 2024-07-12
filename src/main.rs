extern crate clap;
extern crate config;
extern crate log;
extern crate procfs;
extern crate reqwest;
extern crate simplelog;

use clap::{arg, command};
use log::*;
use simplelog::*;
use std::collections::HashMap;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

#[cfg(debug_assertions)]
use std::fs::File;

struct Message {
    command: Option<String>,
    process: Option<i32>,
    exit_status: Option<i32>,
    time_elapsed: Option<Duration>,
}

impl Message {
    fn new() -> Message {
        return Message {
            command: None,
            process: None,
            exit_status: None,
            time_elapsed: None,
        };
    }

    fn generate(&self) -> String {
        let mut message = String::new();

        match &self.command {
            Some(command) => message += &format!("\"{}\"", command),
            None => message += "Command",
        }

        match &self.process {
            Some(process) => message += &format!(" (PID {})", process),
            None => {}
        }

        message += " finished";

        match &self.exit_status {
            Some(exit_status) => {
                if *exit_status == 0 {
                    message += " with success";
                } else {
                    message += &format!(" with status \"{}\"", exit_status);
                }
            }
            None => {}
        }

        match &self.time_elapsed {
            Some(time_elapsed) => message += &format!(" after {:?}", time_elapsed),
            None => {}
        }

        return message;
    }
}

fn send_notification(message: &str, topic: &str) {
    let publish_url = format!("https://ntfy.sh/{}", topic);

    let client = reqwest::blocking::Client::new();
    let _ = client.post(&webhook_url).body(message).send();
}

fn generate_settings_from_matches(matches: &clap::ArgMatches) -> config::Config {
    let mut settings = config::Config::default();
    let config_file = matches.value_of("config").unwrap();
    if std::path::Path::new(config_file).exists() {
        settings
            .merge(config::File::with_name(config_file))
            .unwrap();
    } else {
        settings
            .merge(config::File::with_name("hark").required(false))
            .unwrap();
    }

    if let Some(topic) = matches.value_of("topic") {
        settings.set("topic", topic).unwrap();
    }

    if let Some(command_arguments) = matches.values_of("cmd") {
        settings
            .set("cmd", command_arguments.collect::<Vec<&str>>())
            .unwrap();
    }

    if let Some(process) = matches.value_of("process") {
        settings.set("process", process).unwrap();
    }

    debug!("{}", &format!("Settings {:?}", settings));
    return settings;
}

fn setup_logging() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        #[cfg(debug_assertions)]
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("hark.log").unwrap(),
        ),
    ])
    .unwrap();
    debug!("Logging initalized");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

    let matches = command!()
        .arg(arg!(-v --verbose "Print verbose output"))
        .arg(arg!(-t --topic <TOPIC> "ntfy.sh topic to publish notification to"))
        .arg(arg!(-c --config <FILE> "Sets a custom config file path").default_value("/etc/hark.toml"))
        .arg(arg!(-s --save "Save command line settings to config file"))
        .arg(arg!(-p --process <PID> "PID of the process to monitor"))
        .arg(arg!(<CMD> ... "Command and arguments to be run").trailing_var_arg(true))
        .get_matches();

    let settings = generate_settings_from_matches(&matches);

    // let mut message = Message::new();
    // if let Ok(command_args) = settings.get_array("command") {
    //     let base_cmd = command_args[0].clone().into_str()?;
    //     message.command = Some(base_cmd.clone());

    //     let mut command = Command::new(base_cmd);
    //     for arg in command_args.iter().skip(1) {
    //         command.arg(arg.clone().into_str()?);
    //     }

    //     let command_start = Instant::now();
    //     let mut child = command.spawn().expect("Command failed to start");

    //     let result = child.wait()?;
    //     let command_time_elapsed = command_start.elapsed();

    //     message.exit_status = result.code();
    //     message.time_elapsed = Some(command_time_elapsed);
    // } else if let Ok(processid) = settings.get_int("process") {
    //     message.process = Some(processid as i32);
    //     let process = procfs::process::Process::new(processid as i32)?;

    //     if let Ok(cmdline) = process.cmdline() {
    //         message.command = Some(cmdline[0].clone());
    //     }

    //     while process.is_alive() {
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // }

    // if let Ok(apikey) = settings.get_str("apikey") {
    //     send_notification(&message.generate(), &apikey);
    // } else {
    //     error!("Unable to send notification: no APIKEY given!");
    // }

    return Ok(());
}
