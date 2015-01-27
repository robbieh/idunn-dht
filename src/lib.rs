use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Index, IndexMut};

static NODE_SIZE: usize = 32;

#[derive(Show, Copy)]
pub struct NodeId([u8; 32]);

impl NodeId {
    pub fn new() -> NodeId {
        NodeId([0u8; 32])
    }

    pub fn distance(&self, other: &NodeId) -> NodeId {
        let mut result = NodeId::new();
        for i in (0..NODE_SIZE) { result[i] = self[i] ^ other[i]; }
        result
    }
}

impl Index<usize> for NodeId {
    type Output = u8;

    fn index<'a>(&'a self, index: &usize) -> &'a u8 {
        &self.0[*index]
    }
}

impl IndexMut<usize> for NodeId {
    type Output = u8;

    fn index_mut<'a>(&'a mut self, index: &usize) -> &'a mut u8 {
        &mut self.0[*index]
    }
}

impl PartialEq for NodeId {
    fn eq(&self, other: &NodeId) -> bool {
        !(0..NODE_SIZE).any(|i| self[i] != other[i])
    }
}

impl Eq for NodeId {}

impl PartialOrd for NodeId {
    fn partial_cmp(&self, other: &NodeId) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for NodeId {
    fn cmp(&self, other: &NodeId) -> Ordering {
        self.0.cmp(&other.0)
    }
}
