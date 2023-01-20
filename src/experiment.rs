#[derive(Debug)]
struct Test {
    v: u32,
}

impl Test {
    fn new(v: u32) -> Self {
        Self { v }
    }
}

struct Tests {
    items:Vec<Test>,
}

impl Tests {
    fn new(items: Vec<Test>) -> Self { Self { items } }
    pub fn iter(&self) -> std::slice::Iter<'_, Test> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tests {
    type Item = &'a Test;

    type IntoIter = std::slice::Iter<'a, Test>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}





struct Etest<'a> {
    items: &'a Vec<&'a Test>,
    count: usize,
    pos: usize,
}

impl<'a> Iterator for Etest<'a> {
    type Item = (usize, usize, &'a Test);
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.items.len() {
            let item = self.items[self.count];
            self.count += 1;
            let current_pos = self.pos;
            self.pos += item.v as usize;
            return Some((self.count, current_pos, item));
        }
        None
    }
}

#[derive(Debug)]
struct EnumerateX<I> {
    iter: I,
    count: usize,
}

impl<I> Iterator for EnumerateX<I>
where
    I: Iterator,
{
    // type Item = (usize, I::Item);
    type Item =  I::Item;

    #[inline]
    // fn next(&mut self) -> Option<(usize, I::Item)> {
    fn next(&mut self) -> Option<I::Item> {
        self.iter.next().map(|a| {
            // let ret = (self.count, a);
            let ret = a;
            self.count += 1;
            ret
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests() {
        let items = &Tests::new(vec![Test { v: 111 }, Test { v: 222 }]);
        for item in items {
            println!("item:{:?}", item);
        }
    }

    #[test]
    fn example() {

        let items = &vec![&Test { v: 111 }, &Test { v: 222 }];
        let tests = &Tests::new(vec![Test { v: 111 }, Test { v: 222 }]);
        let test_items = &tests.items;

        let etest = Etest {
            items: items,
            count: 0,
            pos: 0,
        };

        for item in etest {
            println!("item:{:?}", item);
        }
    }
}
