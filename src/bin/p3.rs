// Topic: implementing SinglyLinkedList in RUST

// Definitions

// Rc = To enable multiple ownership, Rust has a type called Rc<T> . Its name is an abbreviation for reference counting,
// which keeps track of the number of references to a value to know whether or not a value is still in use.

// RefCell = RefCell<T> is mainly used in the single-threaded scenario and will give an error if we use in a multithreaded case.
// RefCell<T> checks the mutable borrows at the runtime. Therefore, we can say that we can mutate the value even when the RefCell<T> value is immutable.

use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Debug, Clone)]
struct LinkedList {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl LinkedList {
    pub fn new_empty() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }
}

fn main() {
    let mut item = LinkedList::new_empty();
    item.append("1".to_string());
    item.append("2".to_string());
    println!("{:?}", item);
    item.pop();
    println!("{:?}", item);
}
