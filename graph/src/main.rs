use graph;

use std::env;
use std::io::{self, Write};
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};

struct Stats {
    vertexes_count: usize,
    total_visited: usize,
    console_output_step: usize
}

impl Stats {
    fn new(vertexes_count: usize) -> Stats {
        Stats {
            vertexes_count,
            total_visited: 1,  // `0` node
            console_output_step: 0
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename: String = String::from("data/testdata.graph");
    let filename: &String = if args.len() > 1 { &args[1] } else { &default_filename };
    let graph = graph::utils::read_from_file(filename);

    // simple bfs
    let head = 1;
    let mut stats = Stats::new(graph.vertexes_count);
    println!("searching graph starting from {:?} vertex...", head);
    let mut log = |_v: usize, _parent: usize, _level: i32, _parents: &Vec<usize>| -> bool {
        thread::sleep(time::Duration::from_micros(1)); // simulate processing
        stats.total_visited += 1;
        const CONSOLE_OUTPUT_INTERVAL: usize = 10000;
        let step: usize = stats.total_visited / CONSOLE_OUTPUT_INTERVAL;
        if stats.console_output_step != step {
            stats.console_output_step = step;
            print!("\r - visit {:?} nodes ({:?}%)", stats.total_visited, ((stats.total_visited) * 100) / stats.vertexes_count);
            io::stdout().flush().unwrap();
        }
        return false;
    };
    let result = graph.bfs(head, &mut log);
    println!("\r - visit {:?} nodes ({:?}%)", stats.total_visited, ((stats.total_visited) * 100) / graph.vertexes_count);
    println!("finished searching graph...");
    println!(" - visited vertices:          {:?}", stats.total_visited);
    println!(" - max level:                 {:?}", result.levels.iter().max_by(|x,y| x.cmp(y)).unwrap());
    println!(" - max queue size:            {:?}", result.max_queue_size);

    // parallel bfs
    let head = 1;
    let stats = Arc::new(Mutex::new(Stats::new(graph.vertexes_count)));
    let closure_stats = stats.clone();
    println!("searching graph starting from {:?} vertex...", head);
    let log = move |_v: usize, _parent: usize, _level: i32, _parents: &Vec<usize>| -> bool {
        thread::sleep(time::Duration::from_micros(1)); // simulate processing
        let mut stats = closure_stats.lock().unwrap();
        stats.total_visited += 1;
        const CONSOLE_OUTPUT_INTERVAL: usize = 10000;
        let step: usize = stats.total_visited / CONSOLE_OUTPUT_INTERVAL;
        if stats.console_output_step != step {
            stats.console_output_step = step;
            print!("\r - visit {:?} nodes ({:?}%)", stats.total_visited, ((stats.total_visited) * 100) / stats.vertexes_count);
            io::stdout().flush().unwrap();
        }
        return false;
    };
    let result = graph.bfs_parallel(head, log);
    let stats = stats.lock().unwrap();
    println!("\r - visit {:?} nodes ({:?}%)", stats.total_visited, ((stats.total_visited) * 100) / graph.vertexes_count);
    println!("finished searching graph...");
    println!(" - visited vertices:          {:?}", stats.total_visited);
    println!(" - max level:                 {:?}", result.levels.iter().max_by(|x,y| x.cmp(y)).unwrap());
    println!(" - max queue size:            {:?}", result.max_queue_size);
}
