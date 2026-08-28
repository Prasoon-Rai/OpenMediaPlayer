fn main() {
    println!("Welcome to open media player");
    ffmpeg::init().unwrap();

    let file_path:String = String::new();

    let context = ffmpeg::format::input(&file_path).unwrap();
    let duration = context.duration();

    for stream in context.streams() {
        let codec_parameters = stream.parameters();
        match codec_parameters.medium() {
            ffmpeg::media::Type::Video => {
                let context_decoder = ffmpeg::codec::context::Context::from_parameters(codec_parameters).unwrap();
                let decoder = context_decoder.decoder().video().unwrap();

                let video_resolution = (decoder.width(), decoder.height());
                let frame_rate = stream.avg_frame_rate();

                println!("Resulution of video: {} x {}", video_resolution.0, video_resolution.1);
                println!("Video FPS: {}", frame_rate);
            }
            _ => {}
        }
    }
    println!("Duration of video: {}", duration);
}