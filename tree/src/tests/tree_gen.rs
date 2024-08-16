#[macro_export]
macro_rules! simple_tree {
    () => {{
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

        tree
    }};
}
