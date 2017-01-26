
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
}

fn insert_item(node: &mut Node, item: &Package, parent: &mut Option<Box<Node>>){
  println!("insert item {}", item.height);
  if node.package.is_child(&item){
    println!("is child");
    if node.children.len() == 0 {
      println!("leaf node");
      node.children.push(Node::new(&item));
    } else {
      for child in &mut node.children {
        println!("child {}", child.package.height);
        insert_item(child, item, parent);
      }
    }
  } else if item.is_child(&node.package) {
    println!("is parent");
    let tempNode = node.clone();
    node.package = item.clone();
    node.children.push(tempNode);
  } else {
    println!("is sibling");
    if let Some(ref mut p) = *parent {
      p.children.push(Node::new(&item))
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
    insert_item(&mut self.root, package, &mut None);
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
