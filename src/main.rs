#[macro_use] extern crate nickel;


use chrono::{Local, Timelike};
use nickel::Nickel;

fn greetings() -> &'static str{
    let now = Local::now().time();
    match now.hour(){
        5..=11 => "Good morning",
        13..=16 => "Good afternoon",
        _=> "Good evening",
    }
}

fn main(){
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            greetings()
        } 
    });
    server.listen("127.0.0.1:6767").unwrap();
}
