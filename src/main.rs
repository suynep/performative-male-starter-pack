use clap::Parser;
use dotenv::dotenv;

mod api;
mod config;
mod memoize;
mod responses;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    keyword: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /* Do Initializasionques */
    dotenv().ok();
    let args = Args::parse();
    /* Done Initializasionques */

    let bearer_token = std::env::var("GENIUS_BEARER_TOKEN").unwrap();
    let sr = api::genius_search(args.keyword.to_string(), bearer_token.clone()).await;
    let files = api::get_and_save(args.keyword.to_string(), sr, bearer_token).await;
    let current = memoize::read_already_seen("seen.json".to_string());

    memoize::save_already_seen(args.keyword, &files, current);

    println!("{:?}", files);

    Ok(())
}
