use std::io::{Write, stdout, Error};
use std::process::{Command, Stdio, Child, ExitCode};


use pipewire as pw;
use pw::{
    properties, spa, 
    MainLoop, Context
};
use spa::pod::Pod;
use spa::format::{
    MediaType,
    MediaSubtype,
    FormatProperties,

};
use spa::Direction;

use crate::ffmpeg::FFencoder;

//use crate::ffmpeg::write_ffmpeg;

pub struct UserData {
    format: spa::param::video::VideoInfoRaw,
}

//struct MyStreamEvents {}
//
//impl StreamEvents for MyStreamEvents {
//    fn add_buffer(&mut self, buffer: &pw::Buffer) {
//        // Handle the buffer here
//    }
//}

pub fn init_pipe(id: u32, check: bool) -> Result<(), pw::Error> {
    pw::init();

    let mainloop = MainLoop::new()?;
    let context = Context::new(&mainloop)?;
    let core = context.connect(None)?;

    let data = UserData {
        format: Default::default(),
    };

    let stream = pw::stream::Stream::new(
        &core,
        "video-test",
        properties! {
            *pw::keys::MEDIA_TYPE => "Video",
            *pw::keys::MEDIA_CATEGORY => "Capture",
            *pw::keys::MEDIA_ROLE => "Screen",
        },
    )?;
    //let mut ffchild: Result<Child, std::io::Error> = None;
    let mut ffencoder = FFencoder::new();
    let mut ff_child: Result<Child, Error>;

    let _listener = stream
        .add_local_listener_with_user_data(data)
        .state_changed(|old, new| {
            eprintln!("State changed: {:?} -> {:?}", old, new);
        }).param_changed(move |_, id, user_data, param| {
            let Some(param) = param else {
                return;
            };
            if id != pw::spa::param::ParamType::Format.as_raw() {
                return;
            }

            let (media_type, media_subtype) =
                match pw::spa::param::format_utils::parse_format(param) {
                    Ok(v) => v,
                    Err(_) => return,
                };

            if media_type != MediaType::Video
                || media_subtype != MediaSubtype::Raw
            {
                return;
            }

            user_data
                .format
                .parse(param)
                .expect("Failed to parse param changed to VideoInfoRaw");

            eprintln!("got video format:");
            eprintln!(
                "  format: {} ({:?})",
                user_data.format.format().as_raw(),
                user_data.format.format()
            );
            eprintln!(
                "  size: {}x{}",
                user_data.format.size().width,
                user_data.format.size().height
            );
            eprintln!(
                "  framerate: {}/{}",
                user_data.format.framerate().num,
                user_data.format.framerate().denom
            );
            if check == true {
                std::process::exit(0)
            }

            //let args = ["-y", "-f", "rawvideo", "-pix_fmt", "yuyv422", "-s", "540x480", 
            //    "-r", "30", "-an", "-i", "-", "-c:v", "libx264", "-crf", "24", "test.mkv"
            //    ];
            //ffencoder.args(&args);
            //let ff_child = ffencoder.spawn();
        }).process(|stream, _| {
            match stream.dequeue_buffer() {
                None => eprintln!("out of buffers"),
                Some(mut buffer) => {
                    let datas = buffer.datas_mut();
                    if datas.is_empty() {
                        return;
                    }
                    //let mut buffer = vec![0; w * h * 4];
                    // copy frame data to screen
                    let data = &mut datas[0];
                    let buff: Vec<u8> = data.data().unwrap().to_vec();

                    //let mut stdin = ff_child.unwrap().stdin.as_mut().unwrap();
                    //stdin.write_all(&buff);


                    //ffencoder.stdin(buff);
                    //println!("got a frame of size {}", data.chunk().size());
                    //println!("got a frame of size {:?}", buff);
                    stdout().write_all(&buff).unwrap(); //raw frames to stdout
                }
            }
        }).register()?;


    eprintln!("Created stream {:#?}", stream);

    let obj = pw::spa::pod::object!(
        pw::spa::utils::SpaTypes::ObjectParamFormat,
        pw::spa::param::ParamType::EnumFormat,
        pw::spa::pod::property!(
            FormatProperties::MediaType,
            Id,
            MediaType::Video
        ),
        pw::spa::pod::property!(
            FormatProperties::MediaSubtype,
            Id,
            MediaSubtype::Raw
        ),
        pw::spa::pod::property!(
            FormatProperties::VideoFormat,
            Choice,
            Enum,
            Id,
            pw::spa::param::video::VideoFormat::RGB,
            pw::spa::param::video::VideoFormat::RGB,
            pw::spa::param::video::VideoFormat::RGBA,
            pw::spa::param::video::VideoFormat::RGBx,
            pw::spa::param::video::VideoFormat::BGRx,
            pw::spa::param::video::VideoFormat::YUY2,
            pw::spa::param::video::VideoFormat::I420,
        ),
        pw::spa::pod::property!(
            FormatProperties::VideoSize,
            Choice,
            Range,
            Rectangle,
            pw::spa::utils::Rectangle {
                width: 320,
                height: 240
            },
            pw::spa::utils::Rectangle {
                width: 1,
                height: 1
            },
            pw::spa::utils::Rectangle {
                width: 4096,
                height: 4096
            }
        ),
        pw::spa::pod::property!(
            FormatProperties::VideoFramerate,
            Choice,
            Range,
            Fraction,
            pw::spa::utils::Fraction { num: 25, denom: 1 },
            pw::spa::utils::Fraction { num: 0, denom: 1 },
            pw::spa::utils::Fraction {
                num: 1000,
                denom: 1
            }
        ),
    );
    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();

    let mut params = [Pod::from_bytes(&values).unwrap()];

    stream.connect(
        Direction::Input,
        Some(id),
        pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
        &mut params,
    )?;

    eprintln!("Connected stream");

    mainloop.run();

    Ok(())
}


//TODO: make this 10+bit compatible
//fn generate_frame(buffer: &mut Vec<u8>, width: usize, height: usize, data: Option<u8>) {
//    for row in 0..height {
//        for col in 0..width {
//            buffer[(row * width + col) * 4 + 0] = r; // red
//            buffer[(row * width + col) * 4 + 1] = g; // green
//            buffer[(row * width + col) * 4 + 2] = b; // blue
//            buffer[(row * width + col) * 4 + 3] = a; // alpha
//        }
//    }
//}