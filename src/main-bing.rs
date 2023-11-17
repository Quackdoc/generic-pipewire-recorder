use pipewire::{Context, Core, MainLoop, Stream};
use pipewire::spa::{ReadableDict, VideoFormat, VideoFormatInfo};
use pipewire::types::ObjectType;

fn main() {
    // Initialize PipeWire and run the main loop
    let mainloop = MainLoop::new().expect("failed to get main loop");
    let context = Context::new(&mainloop).expect("failed to get context");
    let core = context.connect(None).expect("failed to get core");
    let registry = core.get_registry().expect("failed to get registry");

    // Create a stream for video capture
    let props = pw_properties_new!(
        PW_KEY_MEDIA_TYPE => "Video",
        PW_KEY_MEDIA_CATEGORY => "Capture",
        PW_KEY_MEDIA_ROLE => "Camera",
    );
    let stream = Stream::new_simple(&mainloop, "video-capture", props).expect("failed to create stream");

    // Add a listener to handle stream events
    let _listener = stream.add_listener_local()
        .param_changed(|id, param| {
            // Handle format changes
            if id == SPA_PARAM_Format {
                if let Some(param) = param {
                    // Parse the video format
                    let format = VideoFormat::parse(param).expect("failed to parse video format");
                    // Print the video format details
                    println!("got video format:");
                    println!(" format: {} ({})", format.info.raw.format, format.info.raw.format.to_string());
                    println!(" size: {}x{}", format.info.raw.size.width, format.info.raw.size.height);
                    println!(" framerate: {}/{}", format.info.raw.framerate.num, format.info.raw.framerate.denom);
                }
            }
        })
        .process(|data| {
            // Handle video frames
            if let Some(buffer) = data.stream.dequeue_buffer() {
                // Get the video buffer
                let video_buffer = buffer.buffer.as_video().expect("failed to get video buffer");
                // Get the video frame data
                let frame_data = video_buffer.data[0].data.expect("failed to get frame data");
                // Print the frame size
                println!("got a frame of size {}", frame_data.len());
                // Queue the buffer back to the stream
                data.stream.queue_buffer(buffer);
            }
        });

    // Enumerate the available video sources
    let _listener = registry.add_listener_local()
        .global(|global| {
            // Filter by video source type
            if global.type_ == ObjectType::Device {
                if let Some(props) = &global.props {
                    if props.get("media.class") == Some("Video/Source") {
                        // Print the video source details
                        println!("found video source: {} ({})", global.id, props.get("device.name").unwrap_or("?"));
                    }
                }
            }
        });

    // Run the main loop
    mainloop.run().expect("failed to run main loop");
}
