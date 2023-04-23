use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
struct Args {
   #[arg(short, long)]
   name: String,
   #[arg(short, long)]
   age: i32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
