extern crate getopts;
use getopts::{ Options, Matches };
use std::env;
mod downloader;

#[derive(Debug)]
struct Args {
    program: String,
    url: Option<String>,
    output: Option<String>,
    parallels: usize,
    help: bool,
}

#[derive(Debug)]
struct Command {
    args: Args,
    usage: String,
}

fn parse_command() -> Command {
    let program: String = env::args().next().unwrap();
    let args: Vec<String> = env::args().skip(1).collect();

    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file", "<NAME>");
    opts.optopt("p", "parallels", "set number of parallels", "<PARALLELS>");
    opts.optflag("h", "help", "help");

    let matches: Matches = opts.parse(&args).unwrap_or_else(|f| panic!(f.to_string()));

    let parallels: usize = match matches.opt_str("p") {
        Some(arg) => { arg.parse::<usize>().unwrap() },
        None => { 1 }
    };

    let url: Option<String> = if matches.free.len() >= 1 { Some(matches.free[0].clone()) } else { None };

    let args = Args {
        program: program.clone(),
        url: url,
        help: matches.opt_present("h"),
        output: matches.opt_str("o"),
        parallels: parallels,
    };

    let brief = format!("Usage: {} URL [options]", program.clone());
    let usage = opts.usage(&brief);

    Command {
        args: args,
        usage: usage,
    }
}

fn main() {
    let command: Command = parse_command();

    if command.args.help {
        println!("{}", command.usage);
        return;
    }

    match command.args.url {
        Some(url) => {
            downloader::download_parallel(&url, &command.args.output, command.args.parallels);
        },
        None => println!("{}", command.usage),
    }
}
