use std::collections::VecDeque;

pub struct SmartList<T> {
    data: Vec<Option<T>>,
    open: VecDeque<u32>,
    cur: u32,
}

impl<T> SmartList<T> {
    // constructor
    fn new() -> SmartList<T> {
        return SmartList {
            data: Vec::new(), //stores the actual data
            open: VecDeque::new(), //stores the open gaps
            cur: 0, //assuming no gaps, first new index; aka capacity
        }
    }

    // add a new element into the SmartList, returns the index that the item was inserted at
    fn add(&mut self, item: T) -> u32 {
        let index_raw: u32 = { //assign index_raw to either the next open gap or the next space
            if self.open.len() > 0 {
                self.open.pop_front().unwrap()
            } else {
                self.cur += 1;
                self.cur - 1
            }
        };
        let index: usize = index_raw.try_into().unwrap();
        self.data[index] = Some(item); //insert item into next open space
        return index_raw;
    }

    // remove an element at the given index
    fn remove(&mut self, index: u32) -> Result<(), ()> {
        let loc: usize = index.try_into().unwrap();
        if index >= self.cur { //if index out of bounds, return Err
            return Err(());
        }
        if let None = self.data[loc] { //if index is None, aka a gap, return Err
            return Err(());
        }
        self.open.push_back(index); //add the new gap into self.open, pop, and return
        self.data[loc] = None;
        return Ok(());
    }
}
