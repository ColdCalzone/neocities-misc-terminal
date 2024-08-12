use super::utils::tree::*;

#[test]
fn trees() {
    let tree: Tree<i8> = Tree::new(3);

    assert_eq!(tree.get_root().get_value(), &3i8);
}
