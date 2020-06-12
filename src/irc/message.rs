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
