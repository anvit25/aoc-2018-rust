use std::fs;
use super::super::utils::{
    dag::DAG,
    workers::Worker
};
use std::collections::HashSet;

fn read_input() -> DAG{
    let input = fs::read_to_string("inputs/7.txt").unwrap();
    let mut dag = DAG::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let from = words.nth(1).unwrap().chars().next().unwrap();
        let to = words.nth(5).unwrap().chars().next().unwrap();
        dag.add_edge(from, to);
    }
    dag
}

pub fn day7a() -> String {
    let mut dag = read_input();
    let mut ans: String = String::new();
    let empty = HashSet::new();
    while let Some(node) = dag.get_min_root_node(&empty) {
        ans.push(node);
        let _ = dag.delete_root_node(node);
    }
    ans
}

pub fn day7b() -> i32 {
    // return 0;
    let mut dag = read_input();
    let mut workers = [Worker::new(); 5];
    let mut time = 0;
    let mut started_nodes = HashSet::new();

    let mut any_busy = workers.iter().any(|w| !w.is_free());
    
    while dag.root_nodes_exist() || any_busy {
        for worker in workers.iter_mut() {
            worker.work();
            if let Some(node) = worker.is_done() {
                let _ = dag.delete_root_node(node);
                assert!(worker.is_free());
            }
        }
        for worker in workers.iter_mut() {
            if worker.is_free() {
                if let Some(node) = dag.get_min_root_node(&started_nodes) {
                    worker.assign(node);
                    started_nodes.insert(node);
                }
            }
        }
        // println!("Time: {}, Workers: {:?}", time, workers);
        time += 1;
        any_busy = workers.iter().any(|w| !w.is_free());
    }
    time-1
}
