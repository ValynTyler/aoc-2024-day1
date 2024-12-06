pub struct Pair {
    pub left: i32,
    pub right: i32,
}

impl Pair {
    pub fn diff(&self) -> i32 {
        (self.right - self.left).abs()
    }
}
