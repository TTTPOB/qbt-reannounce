use clap::Parser;
use parse_duration;
use qbittorrent_web_api::Api;
use std::time::SystemTime;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    server_url: String,
    #[clap(long)]
    only_within: String,
    #[clap(short, long)]
    username: String,
    #[clap(short, long)]
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let api = Api::login(&args.server_url, &args.username, &args.password)
        .await
        .expect("login error");

    let all_torrents = api
        .torrent_management()
        .info()
        .send()
        .await
        .expect("get torrents error");
    let current_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time error")
        .as_secs();
    let within_duration = parse_duration::parse(&args.only_within)
        .expect("parse duration error")
        .as_secs();

    let filtered_torrent_hashes: Vec<&str> = all_torrents
        .iter()
        .filter(|torrent| (current_timestamp - torrent.added_on as u64) < within_duration)
        .map(|torrent| torrent.hash.as_str())
        .collect();
    eprintln!("{} torrents passed filter", filtered_torrent_hashes.len());
    eprintln!("reannouncing");

    api.torrent_management()
        .reannounce(&filtered_torrent_hashes[..])
        .await
        .unwrap();
}
