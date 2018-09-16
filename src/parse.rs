extern crate getopts;
use getopts::{ Options, Matches };
use std::env;

#[derive(Debug)]
pub struct Args {
    pub program: String,
    pub url: Option<String>,
    pub output: Option<String>,
    pub parallels: usize,
    pub help: bool,
}

#[derive(Debug)]
pub struct Command {
    pub args: Args,
    pub usage: String,
}

pub fn parse_command() -> Command {
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
