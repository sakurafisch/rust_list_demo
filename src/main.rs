#![allow(unused)]
use std::{cell::RefCell, fmt::Display, rc::Rc};

type NodeHandle<T> = RefCell<Option<Rc<Node<T>>>>;

struct LinkedList<T> {
    head: NodeHandle<T>,
}

struct Node<T> {
    data: RefCell<T>,
    prev: NodeHandle<T>,
    next: NodeHandle<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: RefCell::new(None),
        }
    }

    fn insert(&self, index: usize, data: T) -> Rc<Node<T>> {
        let mut prev = None;
        let mut this = self.head.clone().into_inner();

        for _ in 0..index {
            prev = this;
            this = prev.as_ref().expect("Out of range").next.clone().into_inner();
        }

        let new = Rc::new(Node {
            data: RefCell::new(data),
            prev: RefCell::new(prev.clone()),
            next: RefCell::new(this.clone()),
        });
        
        *(match &prev {
            Some(p) => &p.next,
            None => &self.head,
        })
        .borrow_mut() = Some(new.clone());

        if let Some(next) = this {
            *next.prev.borrow_mut() = Some(new.clone());
        }

        new
    }

    fn print(&self) where T: Display {
        let mut this: Option<Rc<Node<T>>> = self.head.clone().into_inner();

        print!("head");
        while let Some(rc) = this.as_ref() {
            let rc = rc.clone();
            print!(" -> {}", rc.data.borrow());
            this = rc.next.borrow().clone();
        }

        print!("\n");
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        while let Some(this) = next {
            this.prev.take();
            next = this.next.take();
        }
    }
}

fn main() {
    let mut a = LinkedList::new();
    a.insert(0, 2);
    a.insert(0, 1);
    a.insert(2, 4);
    a.insert(2, 3);
    a.insert(4, 5);
    a.print();
}
