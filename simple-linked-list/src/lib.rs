use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    head: Option<Box<Node<T>>>,
}
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        // Run until the next points to None
        // Why would we want to have the contents a ref and not the option a ref?
        let mut runner_node = self.head.as_ref();
        let mut count: usize = 0;
        while let Some(curr_node) = runner_node {
            count += 1;
            runner_node = curr_node.next.as_ref();
        }

        return count;
    }

    pub fn push(&mut self, element: T) {

        let new_node = Some(Box::new(Node {
            data: element,
            next: None
        }));
        let mut runner_node = &mut self.head;
        let mut count = 0;
        while let Some(curr_node) = runner_node {
            count += 1;
            println!("Made it through {}", count);
            runner_node = &mut curr_node.next;
        }

        match runner_node {
            Some(x) => {println!("has a value");},
            None => {println!("has no value");}
        }

        match runner_node {
            Some(ref mut last) => { last.next = new_node; },
            None => {self.head = new_node;}
        }
            
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
