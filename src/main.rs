use clap::{ArgAction, Parser};
use rebels::app::App;
use rebels::relayer::Relayer;
use rebels::types::AppResult;

#[derive(Parser, Debug)]
#[clap(name="Rebels in the Sky", about = "P(lanet)2P(lanet) basketball", author, version, long_about = None)]
struct Args {
    #[clap(long, short = 's', action=ArgAction::Set, help = "Set random seed for team generation")]
    seed: Option<u64>,
    #[clap(long, short='l', action=ArgAction::SetTrue, help = "Run in local mode (disable networking)")]
    disable_network: bool,
    #[clap(long, short='a', action=ArgAction::SetTrue, help = "Disable audio")]
    disable_audio: bool,
    #[clap(long, short='r', action=ArgAction::SetTrue, help = "Reset all save files")]
    reset_world: bool,
    #[clap(long, short='f', action=ArgAction::SetFalse, help = "Disable generating local teams")]
    generate_local_world: bool,
    #[clap(long, short='n', action=ArgAction::SetTrue, help = "Run in network relayer mode (no game)")]
    relayer_mode: bool,
    #[clap(long, short = 'i', action=ArgAction::Set, help = "Set ip of seed node")]
    seed_ip: Option<String>,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();
    if args.relayer_mode {
        Relayer::new().run().await?;
    } else {
        App::new(
            args.seed,
            args.disable_network,
            args.disable_audio,
            args.generate_local_world,
            args.reset_world,
            args.seed_ip,
        )
        .run()
        .await?;
    }

    Ok(())
}
