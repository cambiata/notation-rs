#[derive(Debug)]
struct VItem {
    value: u32,
}

//--------------------------------------------

struct VItems {
    items: Vec<VItem>,
}

impl VItems {
    fn new(items: Vec<VItem>) -> Self {
        Self { items }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, VItem> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a VItems {
    type Item = &'a VItem;

    type IntoIter = std::slice::Iter<'a, VItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

//--------------------------------------------

// struct ValueSumIterator<'a> {
//     items: &'a Vec<VItem>,
//     count: usize,
//     pos: usize,
// }

// impl<'a> Iterator for ValueSumIterator<'a> {
//     type Item = (usize, usize, &'a VItem);
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < self.items.len() {
//             let item = self.items[self.count];
//             self.count += 1;
//             let current_pos = self.pos;
//             self.pos += item.value as usize;
//             return Some((self.count, current_pos, item));
//         }
//         None
//     }
// }
struct ValueSumIterator<'a> {
    items: &'a Vec<&'a VItem>,
    count: usize,
    pos: usize,
}

impl<'a> Iterator for ValueSumIterator<'a> {
    type Item = (usize, usize, &'a VItem);
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.items.len() {
            let item = self.items[self.count];
            self.count += 1;
            let current_pos = self.pos;
            self.pos += item.value as usize;
            return Some((self.count, current_pos, item));
        }
        None
    }
}

//--------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tests() {
        let v_items = &VItems::new(vec![VItem { value: 111 }, VItem { value: 222 }]);
        for borrowed_v_item in v_items {
            println!("v_item:{:?}", borrowed_v_item);
        }
    }

    #[test]
    fn example() {
        let items = &vec![&VItem { value: 111 }, &VItem { value: 222 }];
        // let tests = &VItems::new(vec![VItem { value: 111 }, VItem { value: 222 }]);
        // let items = &tests.items;

        let custom_iterator = ValueSumIterator {
            items,
            count: 0,
            pos: 0,
        };

        for borrowed_v_item in custom_iterator {
            println!("v_item:{:?}", borrowed_v_item);
        }
    }
}
