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

use std::str::from_utf8;

pub const BUFFER_SIZE: usize = 512;

pub struct Request {
    data: [u8; BUFFER_SIZE],
    messages: Vec<String>,
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

    pub fn messages(&mut self) -> &mut Vec<String> {
        if self.messages.is_empty() {
            for message in self.string().split("\r\n") {
                if message != "" {
                    self.messages.push(message.to_string());
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
}

pub fn new() -> Request {
    Request {
        data: [0 as u8; BUFFER_SIZE],
        messages: Vec::new(),
        size: 0,
    }
}
