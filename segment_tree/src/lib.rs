use std::{cell::RefCell, rc::{Rc, Weak}};

struct Node {
    child: RefCell<Weak<i32>>
}

pub fn func(left: u64, right: u64) -> u64 {
    let node = Node{
        child: RefCell::new(Weak::new())
    };
    *node.child.borrow_mut() = Rc::downgrade(&Rc::new(2));
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = func(2, 2);
        assert_eq!(result, 4);
    }
}
