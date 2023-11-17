# Simple utility to record pipewire streams

this thing sucks right now, However it can be used to record gamescope at the very least.

## Usage

THIS IS TEMPORARY. eventually you will be able to record directly with the tool as well as pipe out, also this doesn't support audio yet.

1. run `pipewire-rec -c` this will print the node number at the end of the line. take note of it. it will also tell you the stream information. you will need that too.

2. run `cargo run -- -t nodeid | ffmpeg cli` you will need to specify pixel format, resolution, and fps. an example is given below

example: `cargo run -- -t 123 | ffmpeg -y -f rawvideo -pix_fmt bgr0 -s 1280x720 -r 60 -an -i - -c:v libx264 -crf 24 test.mkv`
