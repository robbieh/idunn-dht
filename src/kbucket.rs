//this is warning is seriously annoying while trying to develop code
#![allow(dead_code)] 
#![allow(unused_variables)]


//use std::sync::Arc;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

use nodeid::NodeId;

const K: usize = 5;

pub struct NodeInfo {
    nodeid: NodeId,
    prev: Option<Rc<RefCell<NodeInfo>>>,
    next: Option<Rc<RefCell<NodeInfo>>>,
    location: String,
}        

impl fmt::Debug for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "NodeInfo( {}\t", self.nodeid);
        let _ = match self.prev {None => write!(f, "none\t"), Some(ref n) => write!(f, "{}\t", n.borrow().nodeid)};
        let _ = match self.next {None => write!(f, "none\t"), Some(ref n) => write!(f, "{}\t", n.borrow().nodeid)};
        write!(f, "{})", self.location)
    }
}

pub struct KBucket{
    head: Option<Rc<RefCell<NodeInfo>>>,
    tail: Option<Rc<RefCell<NodeInfo>>>,
    tree: BTreeMap<NodeId,Rc<RefCell<NodeInfo>>>
}

impl fmt::Display for KBucket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kbucket")
    }
}

impl KBucket {
    pub fn new() -> KBucket{ 
        KBucket{
            head: Option::None,
            tail: Option::None,
            tree: BTreeMap::<NodeId, Rc<RefCell<NodeInfo>>>::new()
        }
    }

    //always appends to end of linked list
    pub fn kbinsert(&mut self, node: NodeId, loc: String) {
        if self.tail.is_none() {
                let ninfo = NodeInfo{nodeid: node.clone(), 
                    prev: Option::None, next: Option::None, location: loc};
                let ninforc = Rc::new(RefCell::new(ninfo));
                self.head = Option::Some(ninforc.clone());
                self.tail = Option::Some(ninforc.clone());
                self.tree.insert(node, ninforc);
        } else { 
            let ninfo = NodeInfo{nodeid: node.clone(), 
                prev: self.tail.clone(), next: Option::None, location: loc}; 
            let ninforc = Rc::new(RefCell::new(ninfo));
            let oldref = self.tail.clone();
            self.tail = Option::Some(ninforc.clone());
            match oldref 
            {
                Some(rc) => { let mut ni = rc.borrow_mut(); ni.next = Option::Some(ninforc.clone()); }
                None => panic!("Could not insert {:?}", node)
            }
            self.tree.insert(node, ninforc);
        }
    }

    pub fn move_to_top(&mut self, node: NodeId) {
        if self.head.as_ref().unwrap().borrow().nodeid == node {
            return;
        }
        {
            let ref me = self.tree.get(&node).as_ref().unwrap().borrow();
            let ref p = me.prev;
            let ref n = me.next;
            if me.next.is_some() { let ref mut x = me.next.as_ref().unwrap().borrow_mut(); x.prev = p.clone(); };
            if me.prev.is_some() { let ref mut x = me.prev.as_ref().unwrap().borrow_mut(); x.next = n.clone(); };
        }
        {
            let ref me_opt = self.tree.get(&node);
            let ref mut me = me_opt.as_ref().unwrap().borrow_mut();
            //insert this one to top of list
            me.prev = Option::None;
            me.next = self.head.clone();

            let mut x = self.head.as_ref().unwrap().borrow_mut();
            x.prev = Option::Some(me_opt.unwrap().clone());
        }
        {
            let ref mut me_opt = self.tree.get(&node);
            self.head = Option::Some(me_opt.unwrap().clone()); 
        }
    }

    pub fn remove_tail(&mut self) {
        if self.tail.is_some() {
            let ref node = self.tail.as_ref().unwrap().clone();
            let ref id = node.borrow().nodeid.clone();
            let prev = node.borrow().prev.clone();
            println!("removing {:?}", id);
            self.tail = prev;
            self.tree.remove(id);
        }
        if self.tree.len() == 0 {
            self.tail = Option::None;
            self.head = Option::None;
        }
    }

    pub fn sighting(&mut self, node: NodeId, loc: String) {
        let found = self.tree.contains_key(&node);
        if found { 
            self.move_to_top(node); }
        else { self.kbinsert(node, loc);}
    }

    pub fn walk_list(&mut self){
        let ref mut node = self.head.clone();
        let mut c = 0;
        while node.is_some() && c < self.tree.len() {
            c = c + 1;
            println!("{}", node.as_ref().unwrap().borrow().nodeid);
            *node = node.as_ref().unwrap().borrow().next.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nodeid::NodeId;

    #[test]
    fn test_insert() {
        let node0 = NodeId::from_hexdigest("316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf080000").unwrap();
        let node1 = NodeId::from_hexdigest("316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf080001").unwrap();
        let node2 = NodeId::from_hexdigest("316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf080002").unwrap();
        let node3 = NodeId::from_hexdigest("316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf080003").unwrap();
        let loc0 = "localhost:16077".to_string();
        let loc1 = "localhost:16070".to_string();
        let loc2 = "localhost:16071".to_string();
        let loc3 = "localhost:16072".to_string();

        let mut kb1 = KBucket::new();
        kb1.sighting(node0, loc0);
        kb1.sighting(node1, loc1);
        kb1.sighting(node2, loc2);
        kb1.sighting(node3, loc3);

        //sigh. I know this isn't a proper test yet.
        //think I need to make an .iter_list() function or something like that
        //-drh
        kb1.walk_list();


    }

}
