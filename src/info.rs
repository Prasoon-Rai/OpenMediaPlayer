use ffmpeg::Rational;

pub struct VideoInfo {
    resolution: (u32, u32),
    framerate: Rational,
    durationSec: f64,
    durationMin: f64,
}

pub fn get_info(file_path: &str) -> Option<VideoInfo> {
    ffmpeg::init().unwrap();

    let context = ffmpeg::format::input(&file_path).ok()?;
    let duration = context.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64;

    let mut vid_resolution = (0, 0);
    let mut frame_rate = ffmpeg::Rational::new(0, 1);

    for stream in context.streams() {
        let codec_parameters = stream.parameters();
        match codec_parameters.medium() {
            ffmpeg::media::Type::Video => {
                let context_decoder =
                ffmpeg::codec::context::Context::from_parameters(codec_parameters).ok()?;
                let decoder = context_decoder.decoder().video().ok()?;

                vid_resolution = (decoder.width(), decoder.height());
                frame_rate = stream.avg_frame_rate();

            }
            _ => {}
        }
    }

    Some(VideoInfo {
        resolution: vid_resolution,
        durationSec: duration.round(),
        framerate: frame_rate,
        durationMin: ((duration % 3600.0) / 60.0).round(),
    })
}