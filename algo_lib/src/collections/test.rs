use crate::collections::iter_ext::find_count::IterFindCount;
use crate::collections::permutation::Permutation;
use crate::collections::slice_ext::compress::compress;

#[test]
fn test_find() {
    let v = vec![1, 2, 3];
    assert_eq!(Some(1), v.iter().find_eq(&2));
    assert_eq!(None, v.iter().find_eq(&0));
}

#[test]
fn test_permutation_mul() {
    let p = Permutation::new_with_base(vec![1, 3, 2], 1);
    let q = Permutation::new_with_base(vec![2, 1, 3], 1);
    assert_eq!(&p * &q, Permutation::new(vec![2, 0, 1]));
    assert_eq!(&q * &p, Permutation::new(vec![1, 2, 0]));
}

#[test]
fn test_compress() {
    let a = vec![3, 5, 7];
    let b = vec![7, 4, 5];
    assert_eq!(
        compress([&a, &b]),
        (vec![3, 4, 5, 7], [vec![0, 2, 3], vec![3, 1, 2]])
    );
}
