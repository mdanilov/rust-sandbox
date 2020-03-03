//! # Graph 
//! 
//! A library for traversing and searching graph data structures
extern crate num_cpus;
extern crate threadpool;

pub use utils::read_from_file;

pub mod utils;

/// Store graph as CSR (Compressed Sparse Rows)
/// 
/// For an undirected graph with N vertices and M edges, two arrays are needed:
///     X (an array of pointers to adjacent vertices) and
///     A (an array of a list of adjacent vertices).
///
/// The array X stores the beginning and the end of the list of neighbors
/// located in array A, that is, the entire list of neighbors of the vertex J
/// is in array A from the index X [J] to X [J + 1], not including it.
///
/// Array X
/// -----------------------------------------------
/// | 0         | | 1     | | 2         | | 3     |
/// -----------------------------------------------
/// | 0 | 2 | 3 | | 1 | 2 | | 0 | 1 | 2 | | 0 | 3 |
/// -----------------------------------------------
/// Array A
pub struct Graph {
    pub x: Vec<usize>,
    pub a: Vec<usize>,
    pub edges_count: usize,
    pub vertexes_count: usize,
}

pub struct SearchResult {
    pub levels: Vec<i32>,
    pub parents: Vec<usize>,
    pub max_queue_size: usize
}

impl Graph {
    /// Perform breadth-first search algorithm on the graph
    /// starting from the vertex 'v'.
    /// For each explored node the 'cb' callback function is called.
    pub fn bfs<F>(&self, v: usize, delegate: &mut F) -> SearchResult
        where F: FnMut(usize, usize, i32, &Vec<usize>) -> bool {
        use std::cmp;

        let mut levels: Vec<i32> = vec![-1; self.vertexes_count + 1]; // vertex index starts from 1
        let mut parents: Vec<usize> = vec![0; self.vertexes_count + 1]; // vertex index starts from 1
        let mut current_queue: Vec<usize> = Vec::new();
        let mut max_queue_size: usize = 0;
        current_queue.push(v);
        levels[v] = 0;
        let mut level: i32 = 1;
        // If there are no more vertices left in the current queue, the algorithm stops.
        while !current_queue.is_empty() {
            let mut next_queue: Vec<usize> = Vec::new();
            max_queue_size = cmp::max(max_queue_size, current_queue.len());
            // For each vertex from the `current_queue`,
            // we look through the list of neighbors with this vertex `vk`
            // and add to the `next_queue` those that have not yet been marked as visited.
            for vi in current_queue {
                let mut i = self.x[vi];
                while i < self.x[vi + 1] {
                    let vk: usize = self.a[i];
                    // check if the node was not visited yet
                    if levels[vk] == -1 {
                        next_queue.push(vk);
                        parents[vk] = vi;
                        levels[vk] = level; // also mark it as visited
                        if delegate(vk, vi, level, &parents) {
                            return SearchResult { levels, parents, max_queue_size }
                        }
                    }
                    i += 1;
                }
            }
            level += 1;
            current_queue = next_queue;
        }
        SearchResult { levels, parents, max_queue_size }
    }

    pub fn bfs_parallel<F>(&self, v: usize, delegate: F) -> SearchResult
        where F: Fn(usize, usize, i32, &Vec<usize>) -> bool + Send + Sync + 'static {
        use std::cmp;
        use std::sync::mpsc::{channel};
        use threadpool::ThreadPool;
        use std::sync::{Arc};

        let mut levels: Vec<i32> = vec![-1; self.vertexes_count + 1]; // vertex index starts from 1
        let mut parents: Vec<usize> = vec![0; self.vertexes_count + 1]; // vertex index starts from 1
        let mut current_queue: Vec<usize> = Vec::new();
        let mut max_queue_size: usize = 0;
        current_queue.push(v);
        levels[v] = 0;
        let mut level: i32 = 1;

        let pool = ThreadPool::new(num_cpus::get());
        let delegate = Arc::new(delegate);
        let (tx, rx) = channel();

        // If there are no more vertices left in the current queue, the algorithm stops.
        while !current_queue.is_empty() {
            let mut next_queue: Vec<usize> = Vec::new();
            max_queue_size = cmp::max(max_queue_size, current_queue.len());
            for vi in current_queue {
                let mut i = self.x[vi];
                while i < self.x[vi + 1] {
                    let vk: usize = self.a[i];
                    // check if the node was not visited yet
                    if levels[vk] == -1 {
                        next_queue.push(vk);
                        parents[vk] = vi;
                        levels[vk] = level; // also mark it as visited
                        let tx = tx.clone();
                        let parents = parents.clone();
                        let delegate = delegate.clone();
                        pool.execute(move || {
                            tx.send(delegate(vk, vi, level, &parents)).expect("could not send data!");
                        });
                    }
                    i += 1;
                }
            }
            for _ in  0..next_queue.len() {
                let res: bool = rx.recv().unwrap();
                if res {
                    return SearchResult { levels, parents, max_queue_size }
                }
            }
            level += 1;
            current_queue = next_queue;
        }
        SearchResult { levels, parents, max_queue_size }
    }
}
