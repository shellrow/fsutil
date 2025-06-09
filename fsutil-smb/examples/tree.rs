#[macro_use]
extern crate tracing;

use std::path::PathBuf;

use argh::FromArgs;
use fsutil_core::RemoteFileSystem;
#[cfg(target_family = "windows")]
use fsutil_smb::{SmbCredentials, SmbFileSystem};
#[cfg(target_family = "unix")]
use fsutil_smb::{SmbCredentials, SmbFileSystem, SmbOptions};

#[derive(FromArgs)]
#[argh(description = "[smb://address[:port]] on UNIX or \\\\server\\share on Windows")]
struct Args {
    #[argh(option, short = 'P', description = "specify password")]
    password: Option<String>,
    #[cfg(target_family = "windows")]
    #[argh(option, short = 'u', description = "specify username")]
    username: Option<String>,
    #[cfg(target_family = "unix")]
    #[argh(option, short = 'u', description = "specify username")]
    username: String,
    #[cfg(target_family = "unix")]
    #[argh(
        option,
        short = 'w',
        default = r#""WORKGROUP".to_string()"#,
        description = "specify workgroup"
    )]
    workgroup: String,
    #[argh(option, short = 'd', description = "specify directory")]
    dir: PathBuf,
    #[argh(option, short = 's', description = "specify share")]
    share: String,
    #[argh(
        positional,
        description = "smb://address[:port] on UNIX and \\\\server\\share on Windows"
    )]
    server: String,
}

fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let args: Args = argh::from_env();

    let dir = args.dir.clone();

    #[cfg(target_family = "unix")]
    let password = match &args.password {
        Some(p) => p.clone(),
        None => read_secret_from_tty("Password: ").ok().unwrap(),
    };

    #[cfg(target_family = "unix")]
    let mut client = init_client(args, password)?;
    #[cfg(target_family = "windows")]
    let mut client = init_client(args);

    info!("connecting to server...");
    client.connect()?;
    info!("client connected");

    info!("current working directory: {:?}", client.pwd()?);

    info!("listing files at {}", dir.display());
    let files = client.list_dir(&dir)?;

    for file in files {
        println!("{}", file.name());
    }

    info!("disconnecting client...");
    client.disconnect()?;
    info!("client disconnected");

    Ok(())
}

#[cfg(target_family = "windows")]
fn init_client(args: Args) -> SmbFileSystem {
    info!(
        "initializing client with server {} and share {}",
        args.server, args.share
    );
    let mut credentials = SmbCredentials::new(args.server, args.share);
    if let Some(username) = args.username {
        credentials = credentials.username(username);
    }
    if let Some(password) = args.password {
        credentials = credentials.password(password);
    }
    SmbFileSystem::new(credentials)
}

#[cfg(target_family = "unix")]
fn init_client(args: Args, password: String) -> anyhow::Result<SmbFileSystem> {
    info!(
        "initializing client with server {} and share {}, with username {} and workgroup {}",
        args.server, args.share, args.username, args.workgroup
    );
    let client = SmbFileSystem::try_new(
        SmbCredentials::default()
            .server(args.server)
            .share(args.share)
            .username(args.username)
            .password(password)
            .workgroup(args.workgroup),
        SmbOptions::default()
            .one_share_per_server(true)
            .case_sensitive(false),
    )?;

    Ok(client)
}

#[cfg(target_family = "unix")]
/// Read a secret from tty with customisable prompt
fn read_secret_from_tty(prompt: &str) -> std::io::Result<String> {
    rpassword::prompt_password(prompt)
}
