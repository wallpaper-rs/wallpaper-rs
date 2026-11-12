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
        help = "Draw the scene on the graphics card, or take a frame of the picture or video of the package, and keep it, or with --animated a recording of a scene or a video"
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
    #[arg(long, requires = "animated", value_parser = ["gif", "webp"], help = "File type of the recording: gif, or webp, which keeps every color; gif without it")]
    pub format: Option<String>,
    #[arg(
        long,
        requires = "capture",
        value_name = "FILE",
        help = "For a preset package: the installed package file of its base, drawn with the values of the preset"
    )]
    pub base: Option<std::path::PathBuf>,
}
