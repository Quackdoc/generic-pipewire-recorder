use std::process::{Command, Stdio, Child};
use std::io::prelude::*;
use pipewire::spa;
//use stream::

pub struct FFencoder {
    command: Command,
}

impl FFencoder {

    pub fn new() -> FFencoder {
        FFencoder {
            command: Command::new("ffmpeg"),
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut FFencoder {
        self.command.arg(arg);
        self
    }

    pub fn args(&mut self, args: &[&str]) -> &mut FFencoder {
        self.command.args(args);
        self
    }

    pub fn stdin(&mut self, stdin: Stdio) -> &mut FFencoder {
        self.command.stdin(stdin);
        self
    }

    pub fn stdout(&mut self, stdout: Stdio) -> &mut FFencoder {
        self.command.stdout(stdout);
        self
    }

    pub fn stderr(&mut self, stderr: Stdio) -> &mut FFencoder {
        self.command.stderr(stderr);
        self
    }

    pub fn spawn(&mut self) -> std::io::Result<std::process::Child> {
        return self.command.spawn();
    }
    
    pub fn stop(&mut self) {
        std::process::exit(0);
    }

}

//pub fn start_ffmpeg() -> Child {
//    //TODO: make the args configurable
//    let mut child = Command::new("ffmpeg")
//    // Overwrite file if it already exists
//    .arg("-y")
//    // Interpret the information from stdin as "raw video" ...
//    .arg("-f")
//    .arg("rawvideo")
//    // ... where every four bytes are [r, g, b, a] format
//    .arg("-pix_fmt")
//    .arg("yuyv422") //TODO: user_data.format.format().as_raw()
//    // The size of the video is 3840x2160
//    .arg("-s")
//    .arg("640x480") //TODO: user_data.format.size().width,
//    // 60 frames per second
//    .arg("-r")
//    .arg("30") //TODO: user_data.format.framerate().num,
//    // Don't expect any audio in the stream
//    .arg("-an")
//    // Get the data from stdin
//    .arg("-i")
//    .arg("-")
//    // encode to h264
//    .arg("-c:v")
//    .arg("libx264")
//    // variable video bitrate
//    .arg("-crf")
//    .arg("24")
//    // Output file
//    .arg("test.mkv")
//    // stdin, stderr, and stdout are piped
//    .stdin(Stdio::piped())
//    .stderr(Stdio::piped())
//    .stdout(Stdio::piped())
//    // Run the child command
//
//    return child
//}
//
//pub fn handle_raw_data() {
//
//}