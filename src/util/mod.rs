pub trait IteratorExt: Iterator {
    fn single(self) -> Option<Self::Item>;
}

impl<I: Iterator> IteratorExt for I {
    fn single(mut self) -> Option<Self::Item> {
        self.next().and_then(|elem| {
            if self.next().is_none() {
                Some(elem)
            } else {
                None
            }
        })
    }
}
