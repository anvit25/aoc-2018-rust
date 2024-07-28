use std::fmt::{Display, Formatter, Error, Debug};

#[derive(Copy, Clone)]
pub struct Worker {
    node: Option<char>,
    time_left: u32
}

impl Worker {
    pub fn new() -> Worker {
        Worker {node: None, time_left: 0}
    }

    pub fn is_free(&self) -> bool {
        self.node.is_none()
    }

    pub fn assign(&mut self, node: char) {
        self.node = Some(node);
        self.time_left = Self::required_time(node);
    }

    pub fn work(&mut self) {
        if self.time_left > 0 {
            self.time_left -= 1;
        }
    }

    pub fn is_done(&mut self) -> Option<char> {
        if self.time_left == 0 && self.node.is_some() {
            let node = self.node;
            self.free_worker();
            node
        } else {
            None
        }
    }

    pub fn get_node(&self) -> Option<char> {
        self.node
    }

    fn required_time(node: char) -> u32 {
        (node as u32) - 4
    }

    fn free_worker(&mut self) {
        self.node = None;
        self.time_left = 0;
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {})", self.node, self.time_left)
    }
}

impl Debug for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {})", self.node, self.time_left)
    }
}

