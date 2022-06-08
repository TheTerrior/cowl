use std::collections::VecDeque;

// a vector that uses lazy deletion to achieve faster speeds and static indices
pub struct SmartList<T> {
    pub data: Vec<Option<T>>, //the actual vector of data, each element is nullable
    open: VecDeque<usize>, //the list of open spaces within the vector
    cur: usize, //the end index (cannot decrease)
}

impl<T> SmartList<T> {
    // constructor
    pub fn new() -> SmartList<T> {
        return SmartList {
            data: Vec::new(), //stores the actual data
            open: VecDeque::with_capacity(32), //stores the open gaps
            cur: 0, //assuming no gaps, first new index; aka capacity
        }
    }

    pub fn with_capacity(size: usize) -> SmartList<T> {
        return SmartList {
            data: Vec::with_capacity(size),
            open: VecDeque::with_capacity(32),
            cur: 0,
        }
    }

    pub fn len(&mut self) -> usize {
        return self.data.len() - self.open.len();
    }

    // add a new element into the SmartList, returns the index that the item was inserted at
    pub fn add(&mut self, item: T) -> usize {
        if self.open.len() < 1 {
            self.data.push(Some(item)); //push item to the back of the vector if no open spaces
            self.cur += 1;
            return self.cur - 1;
        }
        let index: usize = self.open.pop_front().unwrap();
        self.data[index] = Some(item); //insert item into the next open space
        return index;
    }

    // remove an element at the given index
    pub fn remove(&mut self, index: usize) -> Result<(), ()> {
        if index >= self.cur { //if index out of bounds, return Err
            return Err(());
        }
        if let None = self.data[index] { //if index is None, aka a gap, return Err
            return Err(());
        }
        self.open.push_back(index); //add the new gap into self.open, pop, and return
        self.data[index] = None;
        return Ok(());
    }

    pub fn retrieve(&mut self, index: usize) -> Option<&T> {
        if index > self.data.len() {
            return None;
        }
        match &self.data[index] {
            None => return None,
            Some(_) => return self.data[index].as_ref(),
        }
    }

    pub fn retrieve_mut(&mut self, index: usize) -> Option<&mut T> {
        if index > self.data.len() {
            return None;
        }
        match &self.data[index] {
            None => return None,
            Some(_) => return self.data[index].as_mut(),
        }
    }
}
