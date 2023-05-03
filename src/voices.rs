use crate::voice::*;

#[derive(Debug)]
pub struct Voices {
    pub items: Vec<Voice>,
    pub val: u32,
}

impl Voices {
    pub fn new(items: Vec<Voice>) -> Self {
        let val = 128;
        Self { items, val }
    }
}

#[cfg(test)]
mod tests {
    
    

    #[test]
    fn voices() {}
}
