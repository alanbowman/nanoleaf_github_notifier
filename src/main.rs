mod github;
pub mod nanoleaf;
mod notification;

use clap::{arg, Command};
use github::GithubClient;
use nanoleaf::NanoleafClient;
use notification::Notification;

use std::{thread::sleep, time::Duration};

fn main() {
    let matches = Command::new("Nanoload Control")
        .arg(arg!(--api <API_KEY> "Nanoleaf API Key").required(true))
        .arg(arg!(--host <HOSTNAME> "Hostname/IP").required(true))
        .arg(
            arg!(--port <PORT> "Port")
                .required(true)
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(arg!(--githubapi <API_KEY> "Github API key").required(true))
        .get_matches();

    let api_key = matches.get_one::<String>("api").expect("required");
    let hostname = matches.get_one::<String>("host").expect("required");
    let port = matches.get_one::<u16>("port").expect("required");
    let ghapi = matches.get_one::<String>("githubapi").expect("required");

    let nl = NanoleafClient::new(api_key, format!("http://{}:{}/api/v1", hostname, port));

    let info = nl.get_info();
    info.panel_layout
        .layout
        .position_data
        .iter()
        .for_each(|p| println!("{:?}", p));

    //nl.turn_on();
    //nl.write_command();
    //println!("{}", nl.get_effect());
    // nl.turn_off();

    nl.notify();

    let mut gh = GithubClient::new(ghapi);

    loop {
        let notifications_result = gh.check_for_notifications();

        if let Ok((n, d)) = notifications_result {
            if n > 0 {
                // trigger notification
                nl.notify();
            }
            // should use rate limiting here
            sleep(d);
        } else {
            sleep(Duration::from_secs(20));
        }
    }
}
