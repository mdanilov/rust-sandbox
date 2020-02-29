use graph;

use std::env;
use std::io::{self, Write};

struct Logger {
    vertexes_count: usize,
    total_visited: usize,
    console_output_step: usize
}

impl Logger {
    fn new(vertexes_count: usize) -> Logger {
        Logger {
            vertexes_count,
            total_visited: 1,  // `0` node
            console_output_step: 0
        }
    }
}

impl graph::SearchDelegate for Logger {
    fn entry_node(&mut self, _v: usize, _parent: usize, _level: i32, _parents: &Vec<usize>) -> bool {
        self.total_visited += 1;
        const CONSOLE_OUTPUT_INTERVAL: usize = 100000;
        let step: usize = self.total_visited / CONSOLE_OUTPUT_INTERVAL;
        if self.console_output_step != step {
            self.console_output_step = step;
            print!("\r - visit {:?} nodes ({:?}%)", self.total_visited, ((self.total_visited) * 100) / self.vertexes_count);
            io::stdout().flush().unwrap();
        }
        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename: String = String::from("data/testdata.graph");
    let filename: &String = if args.len() > 1 { &args[1] } else { &default_filename };
    let graph = graph::utils::read_from_file(filename);

    let head = 1;
    let mut stats = Logger::new(graph.vertexes_count);
    println!("searching graph starting from {:?} vertex...", head);
    let result = graph.bfs(head, &mut stats);
    println!("\r - visit {:?} nodes ({:?}%)", stats.total_visited, ((stats.total_visited) * 100) / graph.vertexes_count);
    println!("finished searching graph...");
    println!(" - visited vertices:          {:?}", stats.total_visited);
    println!(" - max level:                 {:?}", result.levels.iter().max_by(|x,y| x.cmp(y)).unwrap());
    println!(" - max queue size:            {:?}", result.max_queue_size);
}
