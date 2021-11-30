use crate::collections::iter_ext::IterFind;

#[test]
fn test_find() {
    let v = vec![1, 2, 3];
    assert_eq!(Some(1), v.iter().find(&2));
    assert_eq!(None, v.iter().find(&0));
}
