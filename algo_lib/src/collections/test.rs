use crate::collections::iter_ext::find_count::IterFindCount;
use crate::collections::slice_ext::compress::compress;
use crate::collections::slice_ext::permutation::Permutation;
use crate::collections::vec_ext::inc_dec::IncDec;

#[test]
fn test_find() {
    let v = vec![1, 2, 3];
    assert_eq!(Some(1), v.iter().find_eq(&2));
    assert_eq!(None, v.iter().find_eq(&0));
}

#[test]
fn test_permutation_mul() {
    let p = vec![1, 3, 2].dec();
    let q = vec![2, 1, 3].dec();
    assert_eq!(p.mul(&q), vec![2, 0, 1]);
    assert_eq!(q.mul(&p), vec![1, 2, 0]);
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
