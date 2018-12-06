use std::cmp::Ordering;

pub trait IteratorExt: Iterator {
    fn single(self) -> Option<Self::Item>;
    fn single_min_by_key<B, F>(self, f: F) -> Option<Self::Item>
    where
        B: Ord,
        F: FnMut(&Self::Item) -> B;
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

    fn single_min_by_key<B, F>(mut self, mut f: F) -> Option<Self::Item>
    where
        B: Ord,
        F: FnMut(&Self::Item) -> B,
    {
        let mut candidate = self.next();
        let mut candidate_key = candidate.as_ref().map(|item| f(item));
        for item in self {
            let key = f(&item);
            if let Some(ref mut candidate_key) = candidate_key {
                match key.cmp(&candidate_key) {
                    Ordering::Less => {
                        *candidate_key = key;
                        candidate = Some(item);
                    }
                    Ordering::Equal => {
                        candidate = None;
                    }
                    Ordering::Greater => {}
                }
            }
        }
        candidate
    }
}
