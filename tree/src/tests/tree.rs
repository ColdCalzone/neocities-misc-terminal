use crate::simple_tree;
use crate::Tree;

// I've never made a macro before
// Not only is this testing the crate, it's testing me.
// Does this need to be a macro? not at all.

// TODO: Macro in tree like this:
// tree! {
//    "Root": {
//        "Left",
//        "Middle": {
//            "Foo",
//            "Bar"
//        },
//        "Right"
//    }
// };
//
// tree! {
//    "/": {
//        "bin",
//        "home": {
//            "guest",
//            "cold"
//        },
//        "etc"
//    }
// };
//

// "is this thing on?"
#[test]
fn trees() {
    let tree: Tree<i8> = Tree::new(3);
    assert_eq!(*tree.get_value(), 3i8);
}

// Ensuring DFS iteration is performed correctly and in the correct order
#[test]
fn trees_dfs_iter() {
    let tree = simple_tree!();
    let v: Vec<&str> = tree.dfs_iter().map(|x| x.as_ref()).collect();

    println!("{:?}", v);

    assert_eq!(*tree.get_value(), "Root");
    assert_eq!(v, vec!["Root", "Left", "L", "M", "R", "Right", "L", "R"]);
}

// Ensuring BFS iteration is performed correctly and in the correct order
#[test]
fn tree_bfs_iter() {
    let tree: Tree<&str> = simple_tree!();

    let v: Vec<&str> = tree.bfs_iter().map(|x| x.as_ref()).collect();

    println!("{:?}", v);

    assert_eq!(*tree.get_value(), "Root");
    assert_eq!(v, vec!["Root", "Left", "Right", "L", "M", "R", "L", "R"]);
}

// Ensuring max_depth works correctly for BFS iteration
#[test]
fn tree_bfs_iter_max_depth() {
    let tree: Tree<&str> = simple_tree!();

    let v: Vec<&str> = tree.bfs_iter().max_depth(1).map(|x| x.as_ref()).collect();

    assert_eq!(*tree.get_value(), "Root");

    assert_eq!(v, vec!["Root", "Left", "Right"]);

    let v: Vec<&str> = tree.bfs_iter().max_depth(2).map(|x| x.as_ref()).collect();

    assert_eq!(v, vec!["Root", "Left", "Right", "L", "M", "R", "L", "R"]);
}

// Ensuring max_depth works correctly for DFS iteration
#[test]
fn tree_dfs_iter_max_depth() {
    let tree: Tree<&str> = simple_tree!();
    let v: Vec<&str> = tree.dfs_iter().max_depth(1).map(|x| x.as_ref()).collect();

    assert_eq!(*tree.get_value(), "Root");

    assert_eq!(v, vec!["Root", "Left", "Right"]);

    let v: Vec<&str> = tree.dfs_iter().max_depth(2).map(|x| x.as_ref()).collect();

    assert_eq!(v, vec!["Root", "Left", "L", "M", "R", "Right", "L", "R"]);
}
