use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    username: String,
}

fn main() {
    let args = Args::parse();

    let url = format!("https://api.github.com/users/{}/events", args.username);
    println!("URL: {}", url);
}
