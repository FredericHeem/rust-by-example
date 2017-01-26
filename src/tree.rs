
#[derive(Copy, Clone)]
struct Package {
  height: i32,
  width: i32,
  depth: i32,
}

impl Package {
  fn is_child(&self, child: &Package) -> bool {
    return (self.height > child.height) && (self.width > child.width) && (self.depth > child.depth);
  }
}

#[derive(Clone)]
struct Node {
  package: Package,
  children: Vec<Node>,
}

impl Node {
  fn new(package: &Package) -> Node{
    Node {package: package.clone(), children: Vec::new()}
  }

  fn insert_item(&mut self, item: &Package, parent: &mut Option<Box<Node>>){
    println!("insert item {}", item.height);
    if self.package.is_child(&item){
      println!("is child");
      if self.children.len() == 0 {
        println!("leaf node");
        self.children.push(Node::new(&item));
      } else {
        for child in &mut self.children {
          println!("child {}", child.package.height);
          child.insert_item(item, parent);
        }
      }
    } else if item.is_child(&self.package) {
      println!("is parent");
      //let tempNode =
/*

      auto tempNode = node;
      Node nodeToAdd(box);
      nodeToAdd.children.push_back(tempNode);
      node = nodeToAdd;
*/
    } else {
      println!("is sibling");

      if let Some(ref mut p) = *parent {
        p.children.push(Node::new(&item))
      }
/*
      match *parent {
        Some(ref mut x) => x.children.push(Node::new(&item)),
        None => println!("no parent"),
      }
*/
    }
  }
}

struct Tree {
  root: Node,
}

impl Tree {
  fn new() -> Tree {
    Tree {root: Node {package: Package {height: 10, width: 10, depth: 10}, children: Vec::new()}}
  }

  fn insert_node(&mut self, package: &Package) {
    self.root.insert_item(package, &mut None);
  }
}


#[cfg(test)]

#[test]
fn box_init() {
  let package = Package {height: 1, width: 2, depth: 5};
  assert!(package.height == 1);
  assert!(package.width == 2);
  assert!(package.depth == 5);
}

#[test]
fn node_init() {
  let package = Package {height: 1, width: 2, depth: 5};
  let node = Node {package: package, children: Vec::new()};
  assert!(node.package.height == 1);
}

#[test]
fn tree_init() {
  let package = Package {height: 1, width: 2, depth: 5};
  let mut tree = Tree::new();
  tree.insert_node(&package);
}
