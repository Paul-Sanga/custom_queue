use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::{Rc, Weak},
};

mod test;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy + Display + Debug + PartialEq + PartialOrd> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct Queue<T> {
    size: usize,
    front: Option<Rc<RefCell<Node<T>>>>,
    back: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy + Display + Debug + PartialEq + PartialOrd> Queue<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            front: None,
            back: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn front(&self) -> Result<Option<T>, String> {
        if self.is_empty() {
            return Err(format!("Queue is empty"));
        }
        Ok(self.front.as_ref().map(|node| node.borrow().value))
    }

    pub fn enqueue(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));
        if self.is_empty() {
            self.front = Some(Rc::clone(&node));
            self.back = Some(Rc::clone(&node));
        }
        self.front.as_ref().map(|front| {
            front.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(Rc::downgrade(&front));
        });
        self.front = Some(node);
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Result<(), String> {
        if self.is_empty() {
            return Err(format!("Queue is empty"));
        }
        if let Some(back) = self.back.take() {
            if let Some(new_back) = back.borrow_mut().next.take() {
                self.back = Some(new_back)
            } else {
                self.back = None;
                self.front = None;
            }
        }
        self.size -= 1;
        Ok(())
    }
}
