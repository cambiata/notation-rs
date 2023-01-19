#[derive(Debug)]
struct Test {
    v: u32,
}

impl Test {
    fn new(v: u32) -> Self {
        Self { v }
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
    type Item = (usize, I::Item);

    #[inline]
    fn next(&mut self) -> Option<(usize, I::Item)> {
        self.iter.next().map(|a| {
            let ret = (self.count, a);
            self.count += 1;
            ret
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {

        let items = &vec![&Test { v: 111 }, &Test { v: 222 }];
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
