use anyhow::Result;
use clap::Args;
use wprs_qr::{Code, Link};

#[derive(Args, Debug)]
pub struct Connect {
    #[arg(help = "Address of the server, https://host[:port][/path]")]
    server: String,
    #[arg(help = "SHA-256 of the server's certificate, as wprs-server fingerprint prints it")]
    fingerprint: String,
    #[arg(long, help = "Invitation code the phone uses to create its account")]
    invite: Option<String>,
    #[arg(long, help = "Also draw the QR code with block characters")]
    draw: bool,
    #[arg(long, help = "Draw for a light terminal background instead of a dark one")]
    light: bool,
}

pub fn run(args: Connect) -> Result<()> {
    let text = Link::new(&args.server, &args.fingerprint, args.invite.as_deref())?.text();
    println!("{text}");
    if args.draw {
        print!("{}", Code::new(&text)?.blocks(args.light));
    }
    Ok(())
}
