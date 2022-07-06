use clap::Parser;

mod menu;
mod pass;
mod task;

#[derive(Parser, Debug)]
struct Cli {
    action: String
}

fn unrecognized(action: String) {
    println!("Unrecognized action: {}", action);
    std::process::exit(1);
}

fn main() {
    let args = Cli::parse();
    match args.action.as_str() {
        "menu" => menu::run(),
        "pass" => pass::run(),
        "task" => task::run(),
        _ => unrecognized(args.action)
    }
}
