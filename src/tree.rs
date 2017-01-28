
#[derive(Copy, Clone)]
struct Package {
    height: i32,
    width: i32,
    depth: i32,
}

impl Package {
    fn new(height: i32, width: i32, depth: i32) -> Package {
        Package {
            height: height,
            width: width,
            depth: depth,
        }
    }
}

trait IsChild<T> {
    fn is_child(&self, child: &T) -> bool;
}

impl IsChild<Package> for Package {
    fn is_child(&self, child: &Package) -> bool {
        return (self.height > child.height) && (self.width > child.width) &&
               (self.depth > child.depth);
    }
}

#[derive(Clone)]
struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
    parent: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(data: &T) -> Node<T> {
        Node {
            data: data.clone(),
            children: Vec::new(),
            parent: None,
        }
    }
}

struct Tree<T> {
    root: Option<Node<T>>,
}

impl<T: Clone> Tree<T> {
    fn new() -> Tree<T> {
        Tree { root: None }
    }

    fn insert_node(&mut self, data: &T) {
        //insert_item(&mut self.root, &package);
    }
}

/*
fn insert_item<T: Clone + IsChild<T> >(node: &mut Node<T>, item: &T){
  println!("insert item ");
  if node.package.is_child(&item){
    println!("is child");
    if node.children.len() == 0 {
      println!("leaf node");
      node.children.push(Node::new(&item));
    } else {
      for child in &mut node.children {
        println!("child");
        //insert_item(child, item);
      }
    }
  } else if item.is_child(&node.package) {
    println!("is parent");
    let tempNode = node.clone();
    node.package = item.clone();
    node.children.push(tempNode);
  } else {
    println!("is sibling");
    /*
    if let Some(ref mut p) = *parent {
      p.children.push(Node::new(&item))
    }
    */
  }
}
*/


#[cfg(test)]
#[test]
fn package_init() {
    let package = Package {
        height: 1,
        width: 2,
        depth: 5,
    };
    assert!(package.height == 1);
    assert!(package.width == 2);
    assert!(package.depth == 5);
}

#[test]
fn package_new() {
    let package = Package::new(1, 2, 5);
    assert!(package.height == 1);
    assert!(package.width == 2);
    assert!(package.depth == 5);
}

#[test]
fn node_init() {
    let node = Node {
        data: Package::new(1, 2, 5),
        children: Vec::new(),
        parent: None,
    };
    assert!(node.data.height == 1);
}

#[test]
fn node_new() {
    let package = Package::new(1, 2, 5);
    let node = Node::new(&package);
    assert!(node.data.height == 1);
}

#[test]
fn tree_init() {
    let package = Package::new(1, 2, 5);
    let mut tree = Tree::new();
    tree.insert_node(&package);

}
