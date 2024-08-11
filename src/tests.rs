#[test]
fn trees() {
    use super::utils::tree::*;
    let mut tree: Tree<String, i8> = Tree::new();

    tree.insert_leaf("Help".into(), 1);
    tree.insert_branch("Test".into(), Tree::new());
    tree.get_branch_mut(&String::from("Test"))
        .unwrap()
        .insert_leaf("Test".into(), -30);

    println!("{:#?}", tree);

    assert_eq!(Some(&1i8), tree.get_leaf(&String::from("Help")));
    assert_eq!(
        Some(&-30i8),
        tree.get_branch(&String::from("Test"))
            .unwrap()
            .get_leaf(&String::from("Test"))
    );
}
