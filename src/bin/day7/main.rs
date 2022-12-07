use orange_trees::{Node, Tree};

fn main() {
  task1();
}

fn task1() {
  let input = std::fs::read_to_string("./src/bin/day7/example.txt")
    .expect("Unable to read the file");

  let mut tree: Tree<&str, u32> = Tree::new(
    Node::new("/", 0)
  );

  input
    .split('\n')
    .fold(tree.root_mut(), |acc, cur| {
      if cur.contains("$ cd ..") {
        let id = acc.parent(acc.id()).expect("No parent").id().clone();
        return acc.query_mut(&id).expect("err");
      } else if cur.contains("$ cd") {
        let id = cur.split_whitespace().last().expect("No last element");
        return acc.query_mut(&id).expect("Invalid child");
      } else if cur.contains("$ ls") {
        return acc;
      } else if cur.contains("dir") {
        let name = cur.split(' ').last().expect("No name for dir");
        
        acc.add_child(Node::new(name, 0));
        
        return acc;
      } else {
        println!("{}", cur);
        let (size, name) = cur.split_once(' ').expect("Invalid split");
        let parsed_size = size.parse::<u32>().expect("Error parsing");

        acc.add_child(Node::new(name, parsed_size));
        
        return acc;
      }
    });

    println!("{:?}", tree);
}