use super::utils::tree::*;

// "is this thing on?"
#[test]
fn trees() {
    let tree: Tree<i8> = Tree::new(3);

    assert_eq!(*tree.get_value(), 3i8);
}

// Ensuring DFS iteration is performed correctly and in the correct order
#[test]
fn trees_dfs_iter() {
    let mut tree: Tree<&str> = Tree::new("Root");

    tree.insert_child_value("Left");
    tree.insert_child_value("Right");

    {
        let left = tree.get_child_mut(0).unwrap();

        left.insert_child_value("L");
        left.insert_child_value("M");
        left.insert_child_value("R");
    }

    {
        let right = tree.get_child_mut(1).unwrap();

        right.insert_child_value("L");
        right.insert_child_value("R");
    }
    //            1 Root
    //          /        \
    //     2 Left        6 Right
    //     /  |  \          /  \
    //  3 L  4 M  5 R     7 L  8 R

    let v: Vec<&str> = tree.dfs_iter().map(|x| x.as_ref()).collect();

    println!("{:?}", v);

    assert_eq!(*tree.get_value(), "Root");
    assert_eq!(v, vec!["Root", "Left", "L", "M", "R", "Right", "L", "R"]);
}

// Ensuring BFS iteration is performed correctly and in the correct order
#[test]
fn tree_bfs_iter() {
    let mut tree: Tree<&str> = Tree::new("Root");

    tree.insert_child_value("Left");
    tree.insert_child_value("Right");

    {
        let left = tree.get_child_mut(0).unwrap();

        left.insert_child_value("L");
        left.insert_child_value("M");
        left.insert_child_value("R");
    }

    {
        let right = tree.get_child_mut(1).unwrap();

        right.insert_child_value("L");
        right.insert_child_value("R");
    }
    //            1 Root
    //          /        \
    //     2 Left        3 Right
    //     /  |  \          /  \
    //  4 L  5 M  6 R     7 L  8 R

    let v: Vec<&str> = tree.bfs_iter().map(|x| x.as_ref()).collect();

    println!("{:?}", v);

    assert_eq!(*tree.get_value(), "Root");
    assert_eq!(v, vec!["Root", "Left", "Right", "L", "M", "R", "L", "R"]);
}

// Ensuring max_depth works correctly for BFS iteration
#[test]
fn tree_bfs_iter_max_depth() {
    let mut tree: Tree<&str> = Tree::new("Root");

    tree.insert_child_value("Left");
    tree.insert_child_value("Right");

    {
        let left = tree.get_child_mut(0).unwrap();

        left.insert_child_value("L");
        left.insert_child_value("M");
        left.insert_child_value("R");
    }

    {
        let right = tree.get_child_mut(1).unwrap();

        right.insert_child_value("L");
        right.insert_child_value("R");
    }
    //1            1 Root
    //           /        \
    //2     2 Left        3 Right
    //    ------------------------
    //      /  |  \          /  \
    //3  4 L  5 M  6 R     7 L  8 R

    let v: Vec<&str> = tree.bfs_iter().max_depth(1).map(|x| x.as_ref()).collect();

    assert_eq!(*tree.get_value(), "Root");

    assert_eq!(v, vec!["Root", "Left", "Right"]);

    let v: Vec<&str> = tree.bfs_iter().max_depth(2).map(|x| x.as_ref()).collect();

    assert_eq!(v, vec!["Root", "Left", "Right", "L", "M", "R", "L", "R"]);
}

// Ensuring max_depth works correctly for DFS iteration
#[test]
fn tree_dfs_iter_max_depth() {
    let mut tree: Tree<&str> = Tree::new("Root");

    tree.insert_child_value("Left");
    tree.insert_child_value("Right");

    {
        let left = tree.get_child_mut(0).unwrap();

        left.insert_child_value("L");
        left.insert_child_value("M");
        left.insert_child_value("R");
    }

    {
        let right = tree.get_child_mut(1).unwrap();

        right.insert_child_value("L");
        right.insert_child_value("R");
    }
    //1            1 Root 1
    //           /        \
    //2     2 Left 2       6 Right 3
    //    ------------------------
    //      /  |  \          /  \
    //3  3 L  4 M  5 R     7 L  8 R

    let v: Vec<&str> = tree.dfs_iter().max_depth(1).map(|x| x.as_ref()).collect();

    assert_eq!(*tree.get_value(), "Root");

    assert_eq!(v, vec!["Root", "Left", "Right"]);

    let v: Vec<&str> = tree.dfs_iter().max_depth(2).map(|x| x.as_ref()).collect();

    assert_eq!(v, vec!["Root", "Left", "L", "M", "R", "Right", "L", "R"]);
}
