// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

//! This file is a rustic interpretation of the the [PipeWire Tutorial 5][tut]
//!
//! tut: https://docs.pipewire.org/page_tutorial5.html

use pipewire as pw;
use pw::{
    properties, spa, 
    MainLoop, Context
};

mod stream;
mod ffmpeg;

use clap::Parser;
use stream::init_pipe;
use std::process::{Command, Stdio, Child};
use std::io::prelude::*;

//struct UserData {
//    format: spa::param::video::VideoInfoRaw,
//}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Specify the stream ID to record
    #[clap(short, long = "stream-id")]
    target: Option<u32>,
    ///Exit after printing the stream info
    #[clap(short, long = "Check-format")]
    check: bool,
}
pub fn main() -> Result<(), pw::Error> {
    let opt = Args::parse();
    let id = opt.target;
    let check = opt.check;
    match id {
        Some(x) => {
            return init_pipe(x, check);
        }
        None => {
            return init_pipe(check_id(), check);
        }
    }    
}


fn check_id() -> u32 {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pw-dump | jq '.[] | select(.info.props[\"media.class\"] == \"Video/Source\") | .info.props.\"node.name\" + \" | \" + .info.props.\"node.description\" + \" | \" + (.id|tostring)' | head -n 40")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<&str> = stdout.split('\n').collect();

    if lines.last() == Some(&"") {
        lines.pop();
    }

    println!("Please select a line:");
    for (i, line) in lines.iter().enumerate() {
        let out = line.replace("\"", "");
        println!("{}. {}", i + 1, out);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let index = input.trim().parse::<usize>().unwrap() - 1;

    let last_word = lines[index].split_whitespace().last().unwrap();
    println!("The last word of the selected line is: {}", last_word.replace("\"", ""));

    let id = last_word.replace("\"", "").parse::<u32>().unwrap();
    return id;
}
