use crate::head::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Heads {
    pub items: Vec<Head>,
}

impl Heads {
    pub fn new(mut items: Vec<Head>) -> Self {
        if items.is_empty() {
            panic!("Heads::new() called with empty vector");
        }
        items.sort_by_key(|item| item.level);
        Self { items }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Head> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Head> {
        self.into_iter()
    }

    pub fn get_level_top(&self) -> i8 {
        self.items[0].level
    }

    pub fn get_level_bottom(&self) -> i8 {
        self.items[self.items.len() - 1].level
    }

    pub fn get_levels(&self) -> Vec<i8> {
        self.items.iter().map(|item| item.level).collect()
    }
}

impl<'a> IntoIterator for &'a Heads {
    type Item = &'a Head;

    type IntoIter = std::slice::Iter<'a, Head>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a> IntoIterator for &'a mut Heads {
    type Item = &'a mut Head;

    type IntoIter = std::slice::IterMut<'a, Head>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

//--------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heads_1() {
        let head1 = Head::from_level(-2);
        let heads = Heads::new(vec![Head::from_level(1), head1]);
        for head in &heads {
            println!("Heads: -- head:{:?}", head);
        }

        println!("heads.items[0];:{:?}", &heads);
    }
}
