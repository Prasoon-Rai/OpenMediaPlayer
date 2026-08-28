fn main() {
    println!("Welcome to open media player");
    let file_path = "test.mp4";

    ffmpeg::init().unwrap();

    let context = ffmpeg::format::input(&file_path).unwrap();
    let duration = context.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64 ;

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
    println!("Duration of video: {} sec", duration.round());
    println!("Duration of video: {} min", ((duration % 3600.0) / 60.0).round());
}