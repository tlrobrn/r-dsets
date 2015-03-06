extern crate disjoint_sets;
use disjoint_sets::DisjointSets;

#[test]
fn constructor() {
    let dsets = DisjointSets::new(42);
    assert_eq!(42, dsets.size());
}

#[test]
fn add_sets() {
    let mut dsets = DisjointSets::new(0);
    dsets.add_sets(42);
    assert_eq!(42, dsets.size());
}


#[test]
fn find_root() {
    let mut dsets = DisjointSets::new(1);
    match dsets.find_root(0) {
        Ok(root) => assert_eq!(0, root),
        Err(e) => panic!(e),
    }
}

#[test]
#[should_fail]
fn find_root_out_of_bounds() {
    let mut dsets = DisjointSets::new(0);
    dsets.find_root(0).ok();
}

#[test]
fn set_union() {
    let mut dsets = DisjointSets::new(42);
    dsets.set_union(0, 21);
    match dsets.find_root(21) {
        Ok(root) => assert_eq!(0, root),
        Err(e) => panic!(e),
    }
}

#[test]
#[should_fail]
fn set_union_out_of_bounds() {
    let mut dsets = DisjointSets::new(1);
    dsets.set_union(0, 1);
}

#[test]
fn size() {
    let dsets = DisjointSets::new(42);
    assert_eq!(42, dsets.size());
}
