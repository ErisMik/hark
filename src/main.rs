extern crate clap;
extern crate reqwest;

use clap::{App, AppSettings, Arg};
use std::collections::HashMap;
use std::process::Command;
use std::time::{Duration, Instant};

struct Message {
    command: Option<String>,
    exit_status: Option<i32>,
    time_elapsed: Option<Duration>,
}

impl Message {
    fn new() -> Message {
        return Message {
            command: None,
            exit_status: None,
            time_elapsed: None,
        };
    }

    fn generate(&self) -> String {
        let mut message = String::new();

        match &self.command {
            Some(command) => message += &format!("\"{}\" finished", command),
            None => message += "Command finished",
        }

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

fn send_notification(message: &str, api_key: &str) {
    let webhook_url = format!(
        "https://maker.ifttt.com/trigger/command_exited/with/key/{}",
        api_key
    );
    let mut webhook_payload = HashMap::new();
    webhook_payload.insert("value1", message);

    let client = reqwest::blocking::Client::new();
    let _ = client.post(&webhook_url).json(&webhook_payload).send();
}

fn main() {
    let matches = App::new("Hark!")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("KEY")
                .help("IFTTT API Key")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("CMD")
                .help("Command and arguments to be run")
                .required(true)
                .multiple(true)
                .index(2),
        )
        .get_matches();

    let api_key = matches.value_of("KEY").unwrap();
    let command_args = matches.values_of("CMD").unwrap().collect::<Vec<_>>();

    let mut message = Message::new();
    message.command = Some(command_args[0].to_string());

    let mut command = Command::new(command_args[0]);
    for arg in command_args.iter().skip(1) {
        command.arg(arg);
    }

    let command_start = Instant::now();
    let mut child = command.spawn().expect("Command failed to start");

    let result = child.wait().unwrap();
    let command_time_elapsed = command_start.elapsed();

    message.exit_status = result.code();
    message.time_elapsed = Some(command_time_elapsed);

    send_notification(&message.generate(), api_key);
}
