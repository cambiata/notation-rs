#[derive(Debug)]
pub struct PairIterator<'a, T> {
    items: &'a Vec<&'a T>,
    idx: usize,
    pos: usize,
}

impl<'a, T> PairIterator<'a, T> {
    fn new(items: &'a Vec<&'a T>) -> Self {
        Self {
            items,
            idx: 0,
            pos: 0,
        }
    }
}

impl<'a, T> Iterator for PairIterator<'a, T> {
    type Item = (Option<&'a T>, Option<&'a T>);
    fn next(&mut self) -> Option<Self::Item> {
        if (self.idx == 0 && self.items.len() < 2) {
            self.idx += 1;
            return Some((Some(self.items[0]), None));
        }
        if self.idx < self.items.len() - 1 {
            let item_left = Some(self.items[self.idx]);
            let item_right = Some(self.items[self.idx + 1]);
            self.idx += 1;
            return Some((item_left, item_right));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::PairIterator;

    #[derive(Debug)]
    pub struct TestItem {
        val: usize,
    }

    #[test]
    fn test3() {
        let vec = vec![&TestItem { val: 111 }, &TestItem { val: 222 }];
        // let vec = vec![&"a"];
        let test = PairIterator::new(&vec);
        for item in test {
            println!("- item:{:?}", item);
        }
        println!("test:{:?}", vec);
    }
}
