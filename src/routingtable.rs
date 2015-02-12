#![allow(dead_code)] 
#![allow(unused_variables)]

use nodeid;
use nodeid::NodeId;
use kbucket::KBucket;

use std::fmt;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Peer {
    pub id: NodeId,
    pub loc: String,
}

pub struct RoutingTable {
    id: NodeId,
    buckets: Vec<KBucket>,
}

impl fmt::Debug for RoutingTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "My id: {:?}", self.id);
        for b in self.buckets.iter().enumerate() {
            writeln!(f,"{}:{:?}",b.0,b.1);
        }
        write!(f,"")
    }
}

impl RoutingTable {
    pub fn new(myid: NodeId) -> RoutingTable {
        let mut rt =
        RoutingTable {
            id: myid,
            buckets: vec![]
        };
        for i in range(0, nodeid::NODE_ID_SIZE * 8) {
            rt.buckets.push(KBucket::new());
        };
        rt
    }

    pub fn sighting(&mut self, node: &NodeId, loc: String) {
        let d = self.id.log_distance(node);
        let ref mut b = self.buckets.get_mut(d).unwrap();
        b.sighting(node.clone(), loc.clone());
    }


    pub fn lookup(& self, node: &NodeId) -> Vec<& Peer> {
        let result = Vec::new();
        let bucket = &self.buckets[self.id.log_distance(node)];
        result
    }
}
