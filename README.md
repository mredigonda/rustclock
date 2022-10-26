<p align="center">
  <img width="300" height="300" src="https://user-images.githubusercontent.com/25920622/198073378-9a9800d0-8fd4-4b4c-afea-cfd426164806.png">
  <h1>RustClock</h1>
</p>

RustClock is an extremely easy-to-use time tracking tool for the command line.

## Demo

![demo](https://user-images.githubusercontent.com/25920622/198069008-0d5ec99a-9c7b-4312-aba8-2243dad57185.gif)

## How to run

For now, you will have to compile it yourself with Cargo.

1. `git clone git@github.com:mredigonda/rustclock.git`
2. `cd rustclock`
3. `cargo build --release`
4. `sudo cp ./target/release/rustclock /usr/local/bin/,rustclock` (or copy it somewhere else in your [PATH](http://www.linfo.org/path_env_var.html#:~:text=PATH%20is%20an%20environmental%20variable,commands%20issued%20by%20a%20user.))
5. Every time you use it, type `,r` and then hit tab, and it should autocomplete to `,rustclock`, then just press enter to run the program!

In the future, I plan to have an easier way to install it.

## How does it work?

It creates entries in a SQLite database on your home folder with the logs of your activities, and lets you add your tasks in a super friendly way.

## Motivation

Command line tools often require we invest a considerable amount of time into learning the different configuration options on how to use them.

While those options are great if we are using the tool in connection with some script that needs to use the program in a very specific way, it's **completely unnecessary** for human users.

RustClock was created as a means to demonstrate how much better it can be. You don't need to know anything about RustClock to start using it, just run it, as many times as you need, and it will work as a conversation with the tool in order to achieve what you want.
