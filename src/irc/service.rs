// Copyright 2020 Jonathan Windle

// This file is part of Platform.

// Platform is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Platform is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with Platform.  If not, see <https://www.gnu.org/licenses/>.

use crate::irc::message::{Connection, Message, Request};
use std::io::Write;
use std::net::Shutdown;
use std::sync::Arc;

pub struct Service {}

impl Service {
    pub fn reply(&self, connection: &Connection, request: &mut Request) {
        // Iterate over each message in the request.
        for message in request.messages() {
            println!("{:} -> {:?}", connection.id(), message.string()); // Remove me later

            // Generate reply based on message command using helper functions.
            let reply = match message.command().to_uppercase().as_ref() {
                "CAP" => self.reply_cap(connection.id(), message),
                "NICK" => self.reply_nick(connection.id(), message),
                "USER" => self.reply_user(connection.id(), message),
                _ => None,
            };

            // If there is a reply write it back to the connection stream.
            match reply {
                Some(reply) => {
                    println!("{:} <- {:?}", connection.id(), reply.string()); // Remove me later

                    // Write reply to connection stream
                    match connection.stream().write(&reply.string().as_bytes()) {
                        Ok(_r) => {}
                        // If we can't write to stream shut it down.
                        Err(_e) => match connection.stream().shutdown(Shutdown::Both) {
                            Ok(_r) => {}
                            Err(_e) => {}
                        },
                    }
                }
                None => {}
            }
        }

        // Flush the connection stream after replies have been written.
        match connection.stream().flush() {
            Ok(_r) => {}
            Err(_e) => {}
        }
    }

    fn reply_cap(&self, _id: String, message: &Message) -> Option<Message> {
        match message.parameters()[0].to_uppercase().as_ref() {
            "LS" => {
                let reply = Message::from_string("CAP * LS : ".to_string());
                Some(reply)
            }
            _ => None,
        }
    }

    fn reply_nick(&self, _id: String, _message: &Message) -> Option<Message> {
        None
    }

    fn reply_user(&self, _id: String, _message: &Message) -> Option<Message> {
        None
    }

    pub fn new() -> Arc<Service> {
        Arc::new(Service {})
    }
}
