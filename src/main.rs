extern crate time;

use std::env;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println! ("Format: {} <sample program>", args[0]);
        return;
    }

    // Warmup
    let mut iters = 10;
    for _ in 0..iters {
        step(&args[1]);
    }
    // Loop with benching
    let sec: u64 = 1000 * 1000 * 1000;
    let time_bound = 1 * sec;
    loop {
        let beg = time::precise_time_ns();
        for _ in 0..iters {
            step(&args[1]);
        }
        let end = time::precise_time_ns();
        let work_time: f64 = ((end - beg) as f64) / (sec as f64);
        if end - beg > time_bound {
            println!("One iteration: {} ms", work_time * 1000.0 / (iters as f64));
            break;
        }
        iters = iters * 2;
    }
}

fn step(exe: &str) {
    Command::new(exe) // .output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn().unwrap_or_else(|e| { panic!("failed to start process: {}", e) })
        .wait().unwrap_or_else(|e| { panic!("failed to join process: {}", e) });
}