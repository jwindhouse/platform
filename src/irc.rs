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

#![allow(dead_code)]

use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::JoinHandle;
use std::{thread, time};

pub const BUFFER_SIZE: usize = 512;

pub struct Listener {
    bind_string: String,
    request_queue: Arc<(Mutex<VecDeque<(TcpStream, Request)>>, Condvar)>,
    run: Arc<RwLock<bool>>,
}

impl Listener {
    pub fn clone_request_queue(&self) -> Arc<(Mutex<VecDeque<(TcpStream, Request)>>, Condvar)> {
        self.request_queue.clone()
    }

    pub fn run(&self) -> JoinHandle<()> {
        // Clone self variables to be moved into new thread
        let bind_string = self.bind_string.clone();
        let request_queue = self.request_queue.clone();
        let run = self.run.clone();

        thread::spawn(move || {
            // Create non-blocking TCP listener
            let listener = TcpListener::bind(bind_string).unwrap();
            listener
                .set_nonblocking(true)
                .expect("Cannot set non-blocking on listener");

            // Sleep this long inside the following while loop
            // to keep CPU cycles low.
            let sleep_time = time::Duration::from_millis(60);

            // Use a VecDeque as a TCPStream queue
            let mut streams = VecDeque::new();

            // While self.run equals true run the loop
            while *run.read().unwrap() {
                // Accept new connections and queue them for later processing
                match listener.accept() {
                    Ok((s, _addr)) => {
                        s.set_nonblocking(true)
                            .expect("Cannot set non-blocking on stream");
                        streams.push_back(s);
                    }
                    Err(_e) => {}
                }

                // Create index counter for while loop
                let mut i = 0;
                while i < streams.len() {
                    // Remove the first stream from the top of the stream queue
                    let mut s = streams.pop_front().unwrap();

                    // Create new IRC request
                    let mut request = Request::new();

                    // Attempt to read data from stream
                    match s.read(request.data()) {
                        Ok(size) => {
                            // Dead streams return valid data but with 0 data size
                            // skip these streams and do not add them back to the
                            // streams queue
                            if size > 0 {
                                // Queue the request for IRC worker threads and notify them
                                let (request_queue, cvar) = &*request_queue;
                                let s_clone = s.try_clone().unwrap();
                                let mut request_queue = request_queue.lock().unwrap();
                                request_queue.push_back((s_clone, request));
                                drop(request_queue);
                                cvar.notify_one();

                                // Put the stream on the back of the stream queue for
                                // later processing
                                streams.push_back(s);
                            }
                        }
                        // If stream would normally block then put stream back on
                        // the stream queue for later processing
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            streams.push_back(s);
                        }
                        Err(e) => {
                            println!("I got error! {:?}", e);
                        }
                    }

                    i = i + 1;
                }

                // Sleep for low CPU cycles
                thread::sleep(sleep_time);
            }
        })
    }

    pub fn set_bind_string(&mut self, string: String) {
        self.bind_string = string;
    }

    pub fn stop(&self) {
        *self.run.write().unwrap() = false;
    }

    pub fn new() -> Listener {
        Listener {
            bind_string: String::new(),
            request_queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
            run: Arc::new(RwLock::new(true)),
        }
    }
}

pub struct Message {
    command: String,
    parameters: Vec<String>,
    string: String,
}

impl Message {
    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn parameters(&self) -> &Vec<String> {
        &self.parameters
    }

    pub fn string(&self) -> &String {
        &self.string
    }

    pub fn from_string(string: String) -> Message {
        let mut parameters = Vec::new();

        {
            let mut buffer = String::new();
            let mut skip = false;
            for c in string.chars() {
                if c == ' ' && !skip {
                    parameters.push(buffer.to_string());
                    buffer.clear();
                } else if c == ':' && !skip {
                    skip = true;
                } else {
                    buffer.push(c);
                }
            }
            parameters.push(buffer.to_string());
        }

        let command = parameters.remove(0);

        Message {
            command: command,
            parameters: parameters,
            string: string,
        }
    }
}

pub struct Request {
    data: [u8; BUFFER_SIZE],
    messages: Vec<Message>,
    size: usize,
}

impl Request {
    pub fn clear_data(&mut self) {
        self.data = [0 as u8; BUFFER_SIZE];
        self.messages.clear();
        self.size = 0;
    }

    pub fn data(&mut self) -> &mut [u8; BUFFER_SIZE] {
        &mut self.data
    }

    pub fn messages(&mut self) -> &mut Vec<Message> {
        if self.messages.is_empty() {
            for message in self.string().split("\r\n") {
                if message != "" {
                    self.messages
                        .push(Message::from_string(message.to_string()));
                }
            }
        }
        &mut self.messages
    }

    pub fn size(&mut self) -> usize {
        let mut size: usize = 0;

        if self.size == 0 {
            for c in &self.data[..] {
                if *c == 0 {
                    break;
                }
                size = size + 1;
            }

            self.size = size;
        }

        self.size
    }

    pub fn string(&mut self) -> String {
        let size = self.size();
        from_utf8(&self.data()[..size]).unwrap().to_string()
    }

    pub fn valid(&mut self) -> bool {
        let size = self.size();
        if size > 2 && self.data[size - 1] == b'\n' && self.data[size - 2] == b'\r' {
            true
        } else {
            false
        }
    }

    pub fn new() -> Request {
        Request {
            data: [0 as u8; BUFFER_SIZE],
            messages: Vec::new(),
            size: 0,
        }
    }
}
