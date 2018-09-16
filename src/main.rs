mod downloader;
mod parse;

fn main() {
    let command: parse::Command = parse::parse_command();

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
