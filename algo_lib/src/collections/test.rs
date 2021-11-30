use crate::collections::iter_ext::IterFind;
use crate::collections::permutation::Permutation;

#[test]
fn test_find() {
    let v = vec![1, 2, 3];
    assert_eq!(Some(1), v.iter().find(&2));
    assert_eq!(None, v.iter().find(&0));
}

#[test]
fn permutation_mul() {
    let p = Permutation::new_with_base(vec![1, 3, 2], 1);
    let q = Permutation::new_with_base(vec![2, 1, 3], 1);
    assert_eq!(&p * &q, Permutation::new(vec![2, 0, 1]));
    assert_eq!(&q * &p, Permutation::new(vec![1, 2, 0]));
}
