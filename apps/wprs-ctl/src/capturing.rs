#[derive(clap::Args, Debug)]
pub struct Capture {
    #[arg(
        required_unless_present = "capture",
        conflicts_with = "capture",
        help = "A picture, or with --animated a gif or webp"
    )]
    pub file: Option<std::path::PathBuf>,
    #[arg(
        long,
        help = "Draw the scene on the graphics card and keep a frame, or with --animated a recording of it"
    )]
    pub capture: bool,
    #[arg(long, requires = "capture", conflicts_with = "animated", value_parser = crate::spanning::second, help = "Second of the scene to keep; its poster time without it")]
    pub time: Option<f64>,
    #[arg(long, requires = "animated", value_parser = crate::spanning::second, help = "Second where the recording starts; 0 without it")]
    pub from: Option<f64>,
    #[arg(long, requires = "animated", help = "Seconds to record, 0.1 to 15; 4 without it")]
    pub seconds: Option<f64>,
    #[arg(long, requires = "animated", help = "Frames a second of the recording, 1 to 30; 10 without it")]
    pub fps: Option<u32>,
}
