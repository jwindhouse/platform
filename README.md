# Platform #

Platform aims to be a communications service designed for both real-time and
asynchronous communication between two or more clients across multiple
modalities such as text, voice, and video.

## Project Goals ##

* Host private communication between two or more clients
* Use and extend (as needed) existing Internet protocols
* Support multiple hardware and operating system platforms
* Develop, run, and scale using commodity hardware
* Provide friendly and feature rich user interfaces
* Create and foster a positive / inclusive development community

## Proposed Protocol Support ##

Protocol support will always be a moving target for this project but the
following are the proposed starting point.

### IRCv3 ###

IRC is a proven one to many communications protocol with ubiquitous client
support. Version 3 of the protocol can be extended making it an ideal candidate
for this project.

[ircv3.net](https://ircv3.net/)

### SMTP ###

SMTP is another proven one to many communications protocol with ubiquitous
client support. It only makes sense to support email as an asynchronous
communication method.

[en.wikipedia.org/wiki/Simple\_Mail\_Transfer_Protocol]
(https://en.wikipedia.org/wiki/Simple_Mail_Transfer_Protocol)

## Building the Project ##

This project is written in [Rust](https://www.rust-lang.org/) and can be built
using cargo.

`cargo build --release`

## Contributing ##

This project needs your help. Please check the
[Issue Tracker](https://github.com/jonathanwindle/platform/issues) for places
to start. Feel free to add your own issues to the tracker if you see something
missing.

### Style Guide ###

This project uses the following styles to keep things consistent amongst
various aspects of the project.

#### Git Branches ####

This project uses the
[git-flow](https://nvie.com/posts/a-successful-git-branching-model/) branching
model.

#### Git Commit Messages ####

Project commit messages should follow the 50/72 style.

[How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/)

#### Rust ####

All code written in Rust should follow the
[Rust Style Guide]
(https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md).
Thankfully the `rustfmt` tool will largely do this for you.

`rustfmt --check src/main.rs`
