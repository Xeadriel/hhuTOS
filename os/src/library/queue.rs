use alloc::boxed::Box;
use core::fmt;
use core::fmt::Display;

/// Generic list element for our linked queue.
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Construct a new node with the given data and no next node.
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

/// A simple linked queue implementation.
/// Elements are enqueued at the end and dequeued from the front.
pub struct LinkedQueue<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedQueue<T> {
    /// Create a new empty linked queue.
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// Enqueue a new element at the end of the queue.
    pub fn enqueue(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        if self.head.is_none() {
            // The queue is empty, so set the head to the new node.
            self.head = Some(new_node);
        } else {
            // Go through the list to find the last node and set its `next` to the new node.
            let mut node = self.head.as_mut().unwrap();

            loop {
                if node.next.is_none() {
                    // We found the last node, so set its `next` to the new node.
                    node.next = Some(new_node);
                    break;
                }

                // Move on with the next node.
                node = node.next.as_mut().unwrap();
            }
        }
    }

    /// Dequeue the first element from the queue and return it.
    pub fn dequeue(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                // The queue is not empty, so we can return the data of the head node.
                // The new head is the next node in the list.
                self.head = old_head.next;
                Some(old_head.data)
            }
            None => None // The queue is empty, so return None.
        }
    }

    /// Remove the first element that matches the given predicate.
    /// Returns true if an element was removed, false otherwise.
    /// `f` is a function that takes a reference to the data and returns true if it matches.
    pub fn remove<F>(&mut self, f: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        // Case 1: head matches
        if let Some(ref node) = self.head {
            if f(&node.data) {
                self.head = self.head.take().and_then(|n| n.next);
                return true;
            }
        }

        // Case 2: search the rest of the list
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            match node.next.as_mut() {
                Some(next_node) if f(&next_node.data) => {
                    node.next = next_node.next.take();
                    return true;
                }
                _ => {
                    current = node.next.as_mut();
                }
            }
        }

        false
    }


}

impl<T: Display> Display for LinkedQueue<T> {
    fn fmt(&self, w: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(w, "[")?;

        let mut node = self.head.as_ref();
        while let Some(current) = node {
            write!(w, "{}", current.data)?;

            node = current.next.as_ref();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }

        write!(w, "]")
    }
}


pub fn test_linked_queue() {
    let mut q = LinkedQueue::new();

    println!("Enqueuing 1, 2, 3");
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    println!("Queue: {}", q);

    println!("Dequeuing: {:?}", q.dequeue());
    println!("Queue: {}", q);

    println!("Enqueuing 1");
    q.enqueue(1);

    println!("Queue: {}", q);

    println!("Removing element == 3");
    q.remove(|x| *x == 3);
    println!("Queue: {}", q);

    println!("Removing element == 100 (not present)");
    let removed = q.remove(|x| *x == 100);
    println!("Removed? {}, Queue: {}", removed, q);
}
