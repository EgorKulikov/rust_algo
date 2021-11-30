pub fn create_order(size: usize) -> Vec<usize> {
    create_order_with_base(size, 0)
}

pub fn create_order_with_base(size: usize, base: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(size);
    for i in base..(size + base) {
        res.push(i);
    }
    res
}
