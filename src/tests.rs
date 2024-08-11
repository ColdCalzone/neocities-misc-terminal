use super::utils::tree::*;

#[test]
fn trees() {
    let mut tree: Tree<i8> = Tree::new();

    tree.get_root_mut().set_value(3);

    assert_eq!(tree.get_root().get_value(), Some(&3i8));
}
