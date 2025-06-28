use jyaml::parse;
use std::env;
use std::fs;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <jyaml-file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    };

    // Create channel for communication
    let (tx, rx) = mpsc::channel();

    // Spawn parsing thread
    let content_clone = content.clone();
    thread::spawn(move || {
        let result = parse(&content_clone);
        let _ = tx.send(result);
    });

    // Wait for result with timeout
    match rx.recv_timeout(Duration::from_secs(3)) {
        Ok(Ok(value)) => {
            println!("✓ {} parsed successfully", filename);
            println!("Result: {:#?}", value);
        }
        Ok(Err(e)) => {
            println!("✗ {} failed to parse: {}", filename, e);
            process::exit(1);
        }
        Err(_) => {
            println!("✗ {} parsing timed out (> 3s)", filename);
            process::exit(1);
        }
    }
}
