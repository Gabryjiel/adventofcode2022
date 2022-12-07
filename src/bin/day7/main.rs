use std::ops::Deref;

use orange_trees::{Node, Tree};

fn main() {
  task1();
}

fn task1() {
  let input = std::fs::read_to_string("./src/bin/day7/example.txt")
    .expect("Unable to read the file");

  let tree: Tree<&str, u32> = Tree::new(
    Node::new("/", 0)
  );

  let mut defa = String::from("/");

  let dirs = vec!["/"];

  let (new_tree, _) = input
    .split('\n')
    .fold((tree, &mut defa), |acc, cur| {
      let (mut tree, position) = acc;

      if cur.contains("$ cd ..") {
        let address = position.split('/').collect::<Vec<_>>();
        let ad = &address[0..address.len()].join("/");

        position.clear();
        position.push_str(ad);

        return (tree, position);
      } else if cur.contains("$ cd") {
        let id = cur.split_whitespace().last().expect("No last element");

        position.push('/');
        position.push_str(id);

        return (tree, position);
      } else if cur.contains("$ ls") {
        return (tree, position);
      } else if cur.contains("dir") {
        let name = cur.split(' ').last().expect("No name for dir");
        let me = position.clone();
        
        let current_node = tree.root_mut().query_mut(&(me.as_str()))
          .expect("Unable to find current node");
        
        current_node.add_child(Node::new(name, 0));
        drop(current_node);
        
        return (tree, position);
      } else {
        let (size, name) = cur.split_once(' ').expect("Invalid split");
        let parsed_size = size.parse::<u32>().expect("Error parsing");

        let current_node = tree.root_mut().query_mut(&(position.clone().as_str()))
          .expect("Unable to find current node");

        current_node.add_child(Node::new(name, parsed_size));
        
        return (tree, position);
      }
    });

    let dirs_with_size = dirs.iter().map(|x| calc_node_value(new_tree.root().query(x).expect("No id")))
    .filter(|size| size < &100000)
    .sum::<u32>();

    println!("{:?}", dirs_with_size);
}

fn calc_node_value(node: &Node<&str, u32>) -> u32 {
  let value = *node.value();

  if value != 0 {
    return value;
  } else {
    return value + node.children().iter().map(|x| calc_node_value(x)).sum::<u32>();
  }
}