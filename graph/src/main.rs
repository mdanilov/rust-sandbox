use graph;

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename: String = String::from("data/testdata.graph");
    let filename: &String = if args.len() > 1 { &args[1] } else { &default_filename };
    let graph = graph::utils::read_from_file(filename);

    let head = 1;
    println!("searching graph starting from {:?} vertex...", head);
    const CONSOLE_OUTPUT_INTERVAL: usize = 100000;
    let mut total_visited: usize = 1; // mark vertex with index 0 visited
    let mut console_output_step: usize = 0;
    let callback = |_v| -> bool {
        total_visited += 1;
        let step: usize = total_visited / CONSOLE_OUTPUT_INTERVAL;
        if console_output_step != step {
            console_output_step = step;
            print!("\r - visit {:?} nodes ({:?}%)", total_visited, ((total_visited) * 100) / graph.vertexes_count);
            io::stdout().flush().unwrap();
        }
        return false;
    };
    let (levels, parents, max_queue_size) = graph.bfs(head, callback);
    println!("\r - visit {:?} nodes ({:?}%)", total_visited, ((total_visited) * 100) / graph.vertexes_count);
    println!("finished searching graph...");
    println!(" - visited vertices:          {:?}", total_visited);
    println!(" - max level:                 {:?}", levels.iter().max_by(|x,y| x.cmp(y)).unwrap());
    println!(" - max queue size:            {:?}", max_queue_size);
}
