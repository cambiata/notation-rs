//================================================================
#[derive(Debug)]
pub struct SomeCloneables<T>
where
    T: Clone,
{
    pub items: Vec<Option<T>>,
}

//----------------------------------------------------------------
pub struct SomeCloneablesRefsIter<'a, T>
where
    T: Clone,
{
    holder: &'a SomeCloneables<T>,
    idx: usize,
    prev_idx: Option<usize>,
}

pub struct SomeCloneablesOwnedIter<T>
where
    T: Clone,
{
    holder: SomeCloneables<T>,
    idx: usize,
    prev_idx: Option<usize>,
}

//----------------------------------------------------------------

impl<'a, T> Iterator for SomeCloneablesRefsIter<'a, T>
where
    T: Clone,
{
    // type Item = (&'a Option<Rc<Test>>, usize, Option<usize>);
    type Item = (Option<T>, usize, Option<T>, Option<usize>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.holder.items.len() {
            None
        } else {
            while self.holder.items[self.idx].is_none() {
                self.idx += 1;
                if self.idx >= self.holder.items.len() {
                    return None;
                }
            }

            let prev = if self.prev_idx.is_some() {
                let prev_idx = self.prev_idx.unwrap();
                let prev_item = self.holder.items[prev_idx].as_ref().unwrap().clone();
                Some(prev_item)
            } else {
                None
            };

            self.idx += 1;

            // let item = &self.test_holder.tests[self.idx - 1];
            let item = self.holder.items[self.idx - 1].clone();

            let result = Some((item, self.idx - 1, prev, self.prev_idx));
            self.prev_idx = Some(self.idx - 1);

            result
        }
    }
}

impl<T> Iterator for SomeCloneablesOwnedIter<T>
where
    T: Clone,
{
    type Item = (Option<T>, usize, Option<T>, Option<usize>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.holder.items.len() {
            None
        } else {
            while self.holder.items[self.idx].is_none() {
                self.idx += 1;
                if self.idx >= self.holder.items.len() {
                    return None;
                }
            }

            let prev = if self.prev_idx.is_some() {
                let prev_idx = self.prev_idx.unwrap();
                let prev_item = self.holder.items[prev_idx].as_ref().unwrap().clone();
                Some(prev_item)
            } else {
                None
            };

            self.idx += 1;

            let item = self.holder.items[self.idx - 1].clone();

            let result = Some((item, self.idx - 1, prev, self.prev_idx));
            self.prev_idx = Some(self.idx - 1);
            result
        }
    }
}

//----------------------------------------------------------------

impl<'a, T> IntoIterator for &'a SomeCloneables<T>
where
    T: Clone,
{
    // type Item = (&'a Option<Rc<Test>>, usize, Option<usize>);
    type Item = (Option<T>, usize, Option<T>, Option<usize>);
    type IntoIter = SomeCloneablesRefsIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        SomeCloneablesRefsIter {
            holder: self,
            idx: 0,
            prev_idx: None,
        }
    }
}

impl<T> IntoIterator for SomeCloneables<T>
where
    T: Clone,
{
    type Item = (Option<T>, usize, Option<T>, Option<usize>);
    type IntoIter = SomeCloneablesOwnedIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        SomeCloneablesOwnedIter {
            holder: self,
            idx: 0,
            prev_idx: None,
        }
    }
}

//================================================================

#[cfg(test)]
mod tests3 {
    #[derive(Debug)]
    pub struct Test {
        val: usize,
    }

    use super::*;
    use std::rc::Rc;

    #[test]
    fn example5() {
        let items: SomeCloneables<Rc<Test>> = SomeCloneables {
            items: vec![
                None,
                Some(Rc::new(Test { val: 11 })), //
                None,
                None,
                Some(Rc::new(Test { val: 44 })), //
                None,
                None,
                None,
                Some(Rc::new(Test { val: 88 })), //
                None,
            ],
        };

        for t in &items {
            dbg!(t);
        }

        for t in items {
            dbg!(t);
        }
    }
}
