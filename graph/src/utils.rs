use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Read graph from the file and store it in the CSR graph
pub fn read_from_file(filename: &String) -> crate::Graph {
    let mut x: Vec<usize> = Vec::new();
    let mut a: Vec<usize> = Vec::new();
    let mut vertexes_total_number: usize = 0;
    let mut edges_total_number: usize = 0;
    const CONSOLE_OUTPUT_INTERVAL: usize = 10000;
    let mut is_header_read: bool = false;
    let mut vertexes_count: usize = 1; // vertex index starts from 1

    let f = File::open(filename).expect("Failed to open graph file");
    let file = BufReader::new(&f);
    for (_num, line) in file.lines().enumerate() {
        let l: String = line.unwrap();
        // ignore lines with comments
        if !l.is_empty() && l.chars().next().unwrap() == '%' {
            continue;
        }
        // first thing is to read the header
        if !is_header_read {
            if l.is_empty() {
                continue;
            }
            let mut substr_iter = l.split_whitespace();
            let mut next_number =
                || -> usize { substr_iter.next().unwrap().parse::<usize>().unwrap() };
            vertexes_total_number = next_number();
            edges_total_number = next_number();
            println!("reading graph...");
            println!(" - vertices: {:?}", vertexes_total_number);
            println!(" - edges:    {:?}", edges_total_number);
            is_header_read = true;
            x = vec![0; vertexes_total_number + 2]; // vertex index starts from 1
                                                    // in an undirected graph, for any pair of vertices,
                                                    // it is necessary to store forward and reverse edges (2 * M)
            a = Vec::with_capacity(2 * edges_total_number);
        } else {
            if vertexes_count > vertexes_total_number {
                panic!(
                    "Invalid file format: vertex count exceed (max: {:?})",
                    vertexes_total_number
                );
            }
            vertexes_count += 1; // advance one vertex
            if 0 == (vertexes_count % CONSOLE_OUTPUT_INTERVAL)
                || vertexes_count >= vertexes_total_number
            {
                print!(
                    "\r - read {:?} nodes ({:?}%)",
                    vertexes_count,
                    ((vertexes_count) * 100) / vertexes_total_number
                );
                io::stdout().flush().unwrap();
            }
            // update CSR model
            if !l.is_empty() {
                // process line (each line represents a vertex)
                // get the edges
                for token in l.split_whitespace() {
                    a.push(token.parse::<usize>().unwrap());
                    if a.len() > 2 * edges_total_number {
                        panic!(
                            "Invalid file format: edge count exceed (max: {:?})",
                            edges_total_number
                        );
                    }
                }
            }
            x[vertexes_count] = a.len();
        }
    }
    let edges_count = a.len();
    print!("\n");
    println!("finished reading graph:");
    println!(" - vertices:                  {:?}", vertexes_count);
    println!(" - individual connections:    {:?}", edges_count);
    crate::Graph {
        x,
        a,
        vertexes_count,
        edges_count,
    }
}
