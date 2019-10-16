extern crate num_cpus;

use clap::{App, Arg};
use expected_value::Config;
use std::process;

fn main() {
    let matches = App::new("Expected value")
        .version("1.0")
        .author("Michał Szczerba <pan.szczerba@protonmail.com>")
        .about("Calculates expected value for maximal subsequence sum of random values sequence with uniform distribution from -1 to 1")
        .arg(Arg::with_name("SAMPLE SIZE")
            .help("Sets sample size")
            .required(true)
            .index(1))
        .arg(Arg::with_name("SAMPLE COUNT")
            .help("Specifies how much samples will be processed in order to calculate their average. Note that it should always be a multiple of used threads")
            .required(true)
            .index(2))
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .value_name("THREAD NUMBER")
            .help("Number of threads used in counting average. If not set defaults to number of avilable processors"))
        .get_matches();

    let sample_size: usize = matches
        .value_of("SAMPLE SIZE")
        .unwrap()
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("Error: Could not parse sample size: {}", e);
            process::exit(1)
        });

    let samples: usize = matches
        .value_of("SAMPLE COUNT")
        .unwrap()
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("Error: Could not parse sample count: {}", e);
            process::exit(1)
        });

    let thread_num: usize = match matches.value_of("threads") {
        Some(s) => s.parse().unwrap_or_else(|e| {
            eprintln!("Error: Could not parse thread number: {}", e);
            process::exit(1)
        }),
        None => num_cpus::get(),
    };

    let config = Config {
        sample_size,
        samples: samples / thread_num,
        thread_num,
    };

    if let Err(e) = expected_value::run(config) {
        eprintln!(
            "Error: Something went wrong during program execution: {}",
            e
        );
        process::exit(1)
    }
}
