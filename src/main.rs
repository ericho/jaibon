extern crate clap;

mod cli;
mod commands;

use std::env;
use std::thread;
use commands::Command;

fn create_nodes_vector(nodes: &str) -> Vec<String> {
    let v: Vec<&str> = nodes.split(',')
        .map(|x| x.trim())
        .collect();
    let mut nodes_vec: Vec<String> = Vec::with_capacity(v.len());
    for node in v {
        nodes_vec.push(node.to_string());
    }
    nodes_vec
}

fn main() {
    let cli = cli::create_cli().get_matches();

    let current_user = env::var("USER").unwrap();
    let user = cli.value_of("user")
        .unwrap_or_else(|| current_user.as_str())
        .to_owned();
    let nodes = cli.value_of("nodes").unwrap();
    let command = cli.value_of("command").unwrap().to_owned();

    let nodes_vec = create_nodes_vector(nodes);

    let mut thread_handlers = Vec::with_capacity(nodes_vec.len());

    for i in nodes_vec {
        let theuser = user.clone();
        let thecmd = command.clone();
        thread_handlers.push(thread::spawn(move || {
            println!("Launching command on node {}", i);
            let mut cmd = Command::new(theuser.to_owned(), &i, thecmd.to_owned());
            cmd.run();
            cmd
        }));
    }

    for t in thread_handlers {
        let cmd = t.join().unwrap();
        println!("==== {} ====", cmd.node);
        match cmd.result {
            Ok(_) => println!("Command result:\n{}", cmd.stdout),
            Err(_) => println!("Error in command:\n{}", cmd.stderr),
        }
    }

    println!("Done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_nodes_vector_with_one_nodes() {
        let r = create_nodes_vector("node1");
        assert_eq!(1, r.len());
        assert_eq!(r, ["node1"]);
    }

    #[test]
    fn create_nodes_vector_with_two_nodes() {
        let r = create_nodes_vector("node1,node2");
        assert_eq!(2, r.len());
        assert_eq!(r, ["node1", "node2"]);
    }

    #[test]
    fn create_nodes_vector_with_space_in_nodes() {
        let r = create_nodes_vector("node1, node2");
        assert_eq!(2, r.len());
        assert_eq!(r, ["node1", "node2"]);
    }

    #[test]
    fn create_nodes_vector_with_space_in_three_nodes() {
        let r = create_nodes_vector("node1, node2,node3");
        assert_eq!(3, r.len());
        assert_eq!(r, ["node1", "node2", "node3"]);
    }

    #[test]
    fn create_nodes_vector_with_lot_of_spaces() {
        let r = create_nodes_vector("node1,      node2,    node3     ,node4");
        assert_eq!(4, r.len());
        assert_eq!(r, ["node1", "node2", "node3", "node4"]);
    }
}
