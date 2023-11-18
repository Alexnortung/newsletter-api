mod methods;
mod newsletters;
use clap::Parser;

/// Simple program to sign an email up for a newsletter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The email address that should be signed up
    #[arg()]
    email: String,

    /// The newsletters the email address should be signed up to
    #[arg(num_args(1..))]
    newsletters: Vec<String>,
}

fn filter_uniq(vec: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    vec.into_iter().filter(|x| seen.insert(x.clone())).collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let newsletters = filter_uniq(args.newsletters);

    Ok(())
}
