use clap::Args;

#[derive(Args, Debug)]
pub struct Copying {
    #[arg(
        long,
        requires = "from",
        help = "Id of the copy; the id of the package followed by .copy when omitted"
    )]
    pub id: Option<String>,
    #[arg(
        long,
        requires = "from",
        help = "Unpack even though its license allows no derived works: you made it or its author allows it"
    )]
    pub mine: bool,
    #[arg(long, requires = "from", help = "Only print what the license allows and the id of the copy")]
    pub facts: bool,
}
