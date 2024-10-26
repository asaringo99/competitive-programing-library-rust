use std::{cell::RefCell, rc::{Rc, Weak}};

pub struct Node {
    pub id: i32,
    pub par: RefCell<Weak<Node>>,
    pub rank: RefCell<i32>,
    pub volume: RefCell<i32>,
    pub edge: RefCell<i32>,
}

impl Node {
    pub fn new(id: i32) -> Rc<Node> {
        let node = Rc::new(Node {
            id,
            par: RefCell::new(Weak::new()),
            rank: RefCell::new(0),
            volume: RefCell::new(1),
            edge: RefCell::new(0),
        });
        *node.par.borrow_mut() = Rc::downgrade(&node);
        node
    }

    pub fn root(self: &Rc<Node>) -> Rc<Node> {
        let par = match self.par.borrow().upgrade() {
            Some(x) => x,
            None => panic!("Node: {} has no parent", self.id)
        };
        if self.id == par.id {
            Rc::clone(self)
        } else {
            *self.par.borrow_mut() = Rc::downgrade(&par.root());
            par
        }
    }

    pub fn merge(self: &Rc<Node>, node: Rc<Node>) {
        if self.root().id == node.root().id {
            *self.edge.borrow_mut() += 1;
            return
        }
        if self.rank == node.rank {
            *self.rank.borrow_mut() += 1;
        }
        *self.volume.borrow_mut() += *node.volume.borrow();
        *node.par.borrow_mut() = Rc::downgrade(self);
    }

    pub fn size(self: &Rc<Node>) -> i32 {
        *self.volume.borrow()
    }

    pub fn is_same(self: &Rc<Node>, node: Rc<Node>) -> bool {
        self.root().id == node.root().id
    }
}

pub struct UnionFind {
    nodes: Vec<Rc<Node>>,
}

impl UnionFind {
    pub fn new(size: i32) -> UnionFind {
        let mut nodes = Vec::new();
        for i in 0..size {
            let node = Node::new(i);
            nodes.push(node);
        }
        UnionFind { nodes: nodes }
    }
}

impl UnionFind {
    pub fn root(&self, k: usize) -> Rc<Node> {
        let node = Rc::clone(&self.nodes[k]);
        let node = node.root();
        node
    }

    pub fn merge(&self, k1: usize, k2: usize) {
        let node1 = Rc::clone(&self.nodes[k1]).root();
        let node2 = Rc::clone(&self.nodes[k2]).root();
        if node1.rank > node2.rank {
            node1.merge(node2);
        } else {
            node2.merge(node1);
        }
    }
    
    pub fn size(&self, k: usize) -> i32 {
        let node = Rc::clone(&self.nodes[k]).root();
        node.size()
    }
    
    pub fn is_same(&self, k1: usize, k2: usize) -> bool {
        let node1 = Rc::clone(&self.nodes[k1]).root();
        let node2 = Rc::clone(&self.nodes[k2]).root();
        node1.is_same(node2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_test() {
        let union_find = UnionFind::new(5);
        union_find.merge(0, 1);
        union_find.merge(1, 2);
        union_find.merge(2, 3);
        union_find.merge(3, 4);
        assert_eq!(union_find.root(3).id, union_find.root(0).id);
    }
    
    #[test]
    fn size_test() {
        let union_find = UnionFind::new(6);
        union_find.merge(0, 1);
        union_find.merge(1, 2);
        union_find.merge(3, 4);
        assert_eq!(union_find.size(1), 3);
        assert_eq!(union_find.size(3), 2);
        assert_eq!(union_find.size(5), 1);
    }
}
