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

use crate::irc::message::Request;
use std::collections::VecDeque;
use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::{sleep, spawn, JoinHandle};
use std::time;

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

        spawn(move || {
            // Create non-blocking TCP listener
            let listener = match TcpListener::bind(bind_string.clone()) {
                Ok(listener) => listener,
                Err(_e) => {
                    panic!("Could not bind to address: {:?}", bind_string);
                }
            };
            listener
                .set_nonblocking(true)
                .expect("Cannot set non-blocking on listener");

            // Sleep this long inside the following while loop
            // to keep CPU cycles low.
            let sleep_time = time::Duration::from_millis(60);

            // Use a VecDeque as a TCPStream queue
            let mut streams = VecDeque::new();

            // While self.run equals true run the loop
            while match run.read() {
                Ok(run) => *run,
                Err(_e) => false,
            } {
                // Accept new connections and queue them for later processing
                match listener.accept() {
                    Ok((s, _addr)) => match s.set_nonblocking(true) {
                        Ok(_r) => {
                            streams.push_back(s);
                        }
                        Err(_e) => {}
                    },
                    Err(_e) => {}
                }

                // Create index counter for while loop
                let mut i = 0;
                while i < streams.len() {
                    // Remove the first stream from the top of the stream queue
                    let mut s = match streams.pop_front() {
                        Some(s) => s,
                        None => {
                            break;
                        }
                    };

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
                                let s_clone = match s.try_clone() {
                                    Ok(s_clone) => s_clone,
                                    Err(_e) => {
                                        i = i + 1;
                                        continue;
                                    }
                                };
                                match request_queue.lock() {
                                    Ok(mut request_queue) => {
                                        request_queue.push_back((s_clone, request));
                                        drop(request_queue);
                                        cvar.notify_one();
                                    }
                                    Err(_e) => {
                                        match run.write() {
                                            Ok(mut run) => {
                                                *run = false;
                                            }
                                            Err(_e) => {}
                                        }
                                        break;
                                    }
                                }

                                // Put the stream on the back of the stream queue for
                                // later processing
                                streams.push_back(s);
                            }
                        }
                        // If stream would normally block then put stream back on
                        // the stream queue for later processing
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            streams.push_back(s);
                        }
                        Err(e) => {
                            println!("I got error! {:?}", e);
                        }
                    }

                    i = i + 1;
                }

                // Sleep for low CPU cycles
                sleep(sleep_time);
            }
        })
    }

    pub fn set_bind_string(&mut self, string: String) {
        self.bind_string = string;
    }

    pub fn stop(&self) {
        match self.run.write() {
            Ok(mut run) => {
                *run = false;
            }
            Err(_e) => {}
        }
    }

    pub fn new() -> Listener {
        Listener {
            bind_string: String::new(),
            request_queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
            run: Arc::new(RwLock::new(true)),
        }
    }
}
