extern crate hyper;
extern crate hyper_tls;
extern crate pbr;
extern crate time;
// use time::PreciseTime;
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;
use std::process;
use std::fs;
use std::io::{BufWriter, Write};
use std::thread;
use std::time::Duration;

pub fn download_parallel(url: &str, output: &Option<String>, parallels: usize) {
    let output = match output {
        Some(value) => value.clone(),
        None => "output_file".to_string(),
    };

    println!("    Download {}", url);
    println!("     Save as {}", output);
    println!("   Parallels {}", parallels);

    let https = HttpsConnector::new(parallels).unwrap();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let fut = client
        .get(url.parse::<hyper::Uri>().unwrap())
        .and_then(|res| {
            let mut f = BufWriter::new(fs::File::create(output).unwrap());
            // let mut count: usize = 0;
            // let mut content_size = 0;
            // let spinner: Vec<char> = "|/-\\".chars().collect();
            // let spinner_length = spinner.len();

            // let start = PreciseTime::now();
            res.into_body().for_each(move |chunk| {
                // let end = PreciseTime::now();
                // let size = chunk.len();
                // content_size += size;
                // print!("\r");
                // print!("           {} {} [bytes] {} [sec]", spinner[count % spinner_length], content_size, start.to(end).num_seconds());
                // count += 1;
                f.write(&chunk).unwrap();
                Ok(())
            })
        })
        .map(|_| {
            // FIXME: I wanna do `f.flush()`. but I don't know how.
            thread::spawn(move || {
                // print!("\n");
                println!("        Done 🎉");
                thread::sleep(Duration::from_millis(100));
                process::exit(0);
            });
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
            process::exit(1);
        });

    rt::run(fut);

}
