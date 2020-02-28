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

impl Graph {
    /// Perform breadth-first search algorithm on the graph
    /// starting from the vertex 'v'.
    /// For each explored node the 'cb' callback function is called.
    pub fn bfs<F>(&self, v: usize, mut cb: F) -> (Vec<i32>, Vec<usize>, usize)
        where F: FnMut(usize) -> bool {
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
                        if cb(vi) {
                            return (levels, parents, max_queue_size)
                        }
                    }
                    i += 1;
                }
            }
            level += 1;
            current_queue = next_queue;
        }
        (levels, parents, max_queue_size)
    }

    pub fn parallel_bfs<F>(&self, v: usize, mut cb: F) -> (Vec<i32>, Vec<usize>, usize)
        where F: FnMut(usize) -> bool {
        use std::cmp;
        use std::sync::mpsc::{channel, RecvError};
        use threadpool::ThreadPool;
        use std::sync::{Arc, RwLock};

        let mut levels: Vec<i32> = vec![-1; self.vertexes_count + 1]; // vertex index starts from 1
        let mut parents: Vec<usize> = vec![0; self.vertexes_count + 1]; // vertex index starts from 1
        let mut current_queue: Vec<usize> = Vec::new();
        let mut max_queue_size: usize = 0;
        const QUEUE_SIZE_PARALLEL_THRESHOLD: usize = 1000;
        current_queue.push(v);
        levels[v] = 0;
        let mut level: i32 = 1;
        const NUM_THREADS: usize = num_cpus::get();
        let pool = ThreadPool::new(NUM_THREADS);

        // If there are no more vertices left in the current queue, the algorithm stops.
        while !current_queue.is_empty() {
            let mut next_queue: Vec<usize> = Vec::new();
            max_queue_size = cmp::max(max_queue_size, current_queue.len());
            // Sort of optimization: run in parallel only if the queue has the meaningful size
            // otherwise run sequential algorithm
            if current_queue.len() < QUEUE_SIZE_PARALLEL_THRESHOLD {
                for vi in current_queue {
                    let mut i = self.x[vi];
                    while i < self.x[vi + 1] {
                        let vk: usize = self.a[i];
                        // check if the node was not visited yet
                        if levels[vk] == -1 {
                            next_queue.push(vk);
                            parents[vk] = vi;
                            levels[vk] = level; // also mark it as visited
                            if cb(vi) {
                                return (levels, parents, max_queue_size)
                            }
                        }
                        i += 1;
                    }
                }
            }
            else {
                // In order to execute one iteration of the algorithm in parallel,
                // you need to create your own nextQueue's for each thread.
                // And after all the cycles are completed, merge the received queues.
                let thread_queue_size = current_queue.len() / NUM_THREADS;
                let (tx, rx) = channel();
                let graph_counter = Arc::new(RwLock::<Graph>::new(self));
                for t in 0..NUM_THREADS {
                    let tx = tx.clone();
                    let begin = t * thread_queue_size;
                    let end = cmp::min(current_queue.len(), (t + 1) * thread_queue_size);
                    let mut cur_queue: Vec<usize> = Vec::new();
                    cur_queue.copy_from_slice(&current_queue[begin..end]);
                    let graph_counter = Arc::clone(&graph_counter);
                    pool.execute(move || {
                        let mut next_queue: Vec<(usize, usize, i32)> = Vec::new();
                        for vi in &cur_queue {
                            let graph = graph_counter.read().unwrap();
                            for i in graph.x[*vi]..graph.x[*vi + 1] {
                                let vk: usize = graph.a[i];
                                // check if the node was not visited yet
                                if levels[vk] == -1 {
                                    next_queue.push((vk, *vi, level));
                                }
                            }
                        }
                        tx.send(next_queue).expect("Could not send data!");
                    });
                }
                // wait until all threads finish their job
                // and merge the results
                for _ in  0..NUM_THREADS {
                    let queue: Vec<(usize, usize, i32)> = rx.recv().unwrap();
                    for (vk, vi, level) in queue {
                        parents[vk] = vi;
                        levels[vk] = level; // also mark it as visited
                        next_queue.push(vk);
                    }
                }
            }
            level += 1;
            current_queue = next_queue;
        }
        (levels, parents, max_queue_size)
    }
}
