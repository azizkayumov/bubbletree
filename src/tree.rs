pub struct BubbleTree {
    fanout: usize,
}

impl BubbleTree {
    #[must_use]
    pub fn new(fanout: usize) -> Self {
        BubbleTree { fanout }
    }
}
