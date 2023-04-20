use crate::head::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Heads {
    items: Vec<Head>,
}

impl Heads {
    pub fn new(items: Vec<Head>) -> Self {
        // items.sort_by_key(|item| item.level);
        Self { items }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Head> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Head> {
        self.into_iter()
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
    // use super::Heads;
    // use super::Heads;

    #[test]
    fn test_Heads() {
        let head0 = Head::new(1, HeadAttributes { accidental: None });
        let head1 = Head::new(-2, HeadAttributes { accidental: None });
        let vec = vec![Head::new(1, HeadAttributes { accidental: None }), head1];
        let heads = Heads::new(vec);
        for head in &heads {
            println!("Heads: -- head:{:?}", head);
        }

        println!("heads.items[0];:{:?}", &heads);
    }


}
