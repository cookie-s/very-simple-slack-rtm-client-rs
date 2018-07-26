#![feature(box_syntax, box_patterns)]

extern crate config;
extern crate slack;

use std::thread;
use std::io::Write;
use std::collections::HashMap;

mod slack_handler;

fn main() {
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name("config").required(false))
        .unwrap();

    let table = config.get_table("tokens").unwrap();
    let mut tokens = HashMap::new();
    for (team, val) in table {
        let token = val.into_str().unwrap();
        tokens.insert(team, token);
    }

    println!("{:#?}", tokens.keys());
    println!("ok?");
    {
        let mut l = String::new();
        std::io::stdin().read_line(&mut l).unwrap();
    }

    let mut children = vec![];
    for (team, token) in tokens {
        children.push(thread::spawn(move || {
            let rtm = slack::RtmClient::login(&token).unwrap();
            let mut handler = slack_handler::VerySimpleHandler {
                team: &team,
                channels: HashMap::new(),
                users: HashMap::new(),
            };
            let res = rtm.run(&mut handler);
            let stdout = std::io::stdout();
            let _ = writeln!(&mut stdout.lock(), "handler for {} ended.", team);
            res
        }));
    }

    for child in children {
        let join = child.join();
        let stdout = std::io::stdout();
        let _ = writeln!(&mut stdout.lock(), "{:?}", join);
    }
}
