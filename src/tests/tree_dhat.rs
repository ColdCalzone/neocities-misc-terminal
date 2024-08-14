use crate::simple_tree;
use crate::utils::tree::*;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn tree_dhat() {
    let _profiler = dhat::Profiler::new_heap();

    let t: Tree<&str> = simple_tree!();

    assert_eq!(*t.get_value(), "Root");
}
