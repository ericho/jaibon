extern crate clap;

mod cli;
mod commands;

use std::env;
use std::thread;
use commands::Command;
use commands::CommandErrors;

fn create_nodes_vector(nodes: &str) -> Vec<&str> {
    let v: Vec<&str> = nodes.split(',').map(|x| x.trim()).collect();
    v
}

fn create_commands_vector(user: &str,
                          nodes_vec: &Vec<&str>,
                          command: &str) -> Vec<Command> {
    let mut vectors: Vec<Command> = Vec::new();
    for x in nodes_vec {
        vectors.push(Command::new(user, x, command));
    }
    vectors
}

fn main() {
    let cli = cli::create_cli().get_matches();

    let current_user = env::var("USER").unwrap();
    let user = cli.value_of("user").unwrap_or_else(|| current_user.as_str());

    let nodes = cli.value_of("nodes").unwrap();
    let command = cli.value_of("command").unwrap();

    let nodes_vec = create_nodes_vector(nodes);

    let mut commands = create_commands_vector(&user, &nodes_vec, &command);

//    let mut threads = Vec::new();

    // for c in &mut commands {
    //     threads.push(thread::spawn(move || {
    //         c.run();
    //     }));
    // }

    // for t in threads {
    //     let _ = t.join();
    // }

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
