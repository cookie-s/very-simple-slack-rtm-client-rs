extern crate std;
extern crate slack;

use std::io::Write;
use std::collections::HashMap;

pub struct VerySimpleHandler<'a> {
    pub team: &'a str,
    pub channels: HashMap<String, slack::api::Channel>,
    pub users: HashMap<String, slack::api::User>,
}

impl<'a> VerySimpleHandler<'a> {
    fn get_chan_name(&self, chan_id: &String) -> String {
        if let Some(chan) = self.channels.get(chan_id) {
            if let Some(chan_name) = &chan.name {
                return chan_name.clone();
            }
        }
        format!("<{}>", chan_id)
    }
    fn get_user_name(&self, user_id: &String) -> String {
        if let Some(user) = self.users.get(user_id) {
            if let Some(user_name) = &user.name {
                return user_name.clone();
            }
        }
        format!("<{}>", user_id)
    }
}

impl<'a> slack::EventHandler for VerySimpleHandler<'a> {
    fn on_connect(&mut self, cli: &slack::RtmClient) {
        let info = cli.start_response();
        {
            if let Some(channels) = &info.channels {
                for chan in channels {
                    if let Some(id) = &chan.id {
                        self.channels.insert(id.clone(), chan.clone());
                    };
                }
            }
        }
        {
            if let Some(users) = &info.users {
                for u in users {
                    if let Some(id) = &u.id {
                        self.users.insert(id.clone(), u.clone());
                    }
                }
            }
        }

        let stdout = std::io::stdout();
        let _ = writeln!(&mut stdout.lock(), "[{}] connected.", self.team);
    }

    fn on_close(&mut self, _: &slack::RtmClient) {
        let stdout = std::io::stdout();
        let _ = writeln!(&mut stdout.lock(), "[{}] connection closed.", self.team);
    }

    fn on_event(&mut self, _: &slack::RtmClient, event: slack::Event) {
        let stdout = std::io::stdout();

        match event {
            slack::Event::UserTyping {
                channel: ref chan_id,
                user: ref user_id,
            } => {
                let chan_name = self.get_chan_name(chan_id);
                let user_name = self.get_user_name(user_id);

                let _ = writeln!(
                    &mut stdout.lock(),
                    "[{}:{}] Typing {}",
                    self.team,
                    chan_name,
                    user_name,
                );
            }
            slack::Event::Message(box msg) => match msg {
                slack::Message::Standard(slack::api::MessageStandard {
                    channel: ref chan_id,
                    user: ref user_id,
                    text: Some(ref text),
                    ..
                }) => {
                    let chan_name = chan_id
                        .clone()
                        .map_or(String::from("<no>"), |cid| self.get_chan_name(&cid));
                    let user_name = user_id
                        .clone()
                        .map_or(String::from("<no>"), |uid| self.get_user_name(&uid));

                    let _ = writeln!(
                        &mut stdout.lock(),
                        "[{}:{}] StandardMessage {}: {}",
                        self.team,
                        chan_name,
                        user_name,
                        text,
                    );
                }
                _ => (),
            },
            slack::Event::ChannelMarked { .. } => (),
            _ => {
                let _ = writeln!(&mut stdout.lock(), "[{}] {:?}", self.team, event);
            }
        };
    }
}
