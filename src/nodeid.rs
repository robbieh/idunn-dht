#![allow(dead_code)] 
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::fmt;
use std::num::Int;

use util;

const NODE_ID_SIZE: usize = 32;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeId([u8; NODE_ID_SIZE]);

impl NodeId {
    pub fn from_hexdigest(hex: &str) -> Option<NodeId> {
        if hex.len() != 2 * NODE_ID_SIZE { return None; }
        let mut result = [0u8; NODE_ID_SIZE];
        for (i, bs) in hex.as_bytes().chunks(2).enumerate() {
            result[i] = match util::parse_hex_bytes(bs) {
                None => { return None; },
                Some(b) => { b as u8 },
            }
        }
        Some(NodeId(result))
    }

    #[inline]
    pub fn distance_cmp(&self, nid1: &NodeId, nid2: &NodeId) -> Ordering {
        for i in (0..NODE_ID_SIZE) {
            let b = self.0[i];
            let order = (b ^ nid1.0[i]).cmp(&(b ^ nid2.0[i]));
            if order != Ordering::Equal { return order; }
        }
        return Ordering::Equal
    }

    pub fn log_distance(&self, other: &NodeId) -> usize {
        let mut bucket = (NODE_ID_SIZE * 8) - 1;
        for i in (0..NODE_ID_SIZE) {
            let b = (self.0[i] ^ other.0[i]).leading_zeros();
            bucket -= b;
            if b < 8 { break; }
        }
        return bucket;
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &b in self.0.iter() { try!(write!(f, "{:02x}", b)); }
        Ok(())
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeId[{}]", self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hexdigest() {
        let digest =
            "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7c";
        let nid = NodeId::from_hexdigest(digest);
        assert!(nid.is_some());
        let nid = nid.unwrap();
        assert_eq!(format!("{}", nid), digest);
    }

    #[test]
    fn test_order() {
        let nid1 = NodeId::from_hexdigest(
            "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7c")
            .unwrap();
        let nid2 = NodeId::from_hexdigest(
            "dd3311cda63ca68104bcd392bfa0e26d2f911b6f7ab20f505ab9636fe2094e3f")
            .unwrap();
        assert!(nid1 < nid2);
        assert!(nid2 > nid1);
        assert!(nid1 != nid2);
        assert!(nid1 == nid1);
    }

    #[test]
    fn test_distance_comp() {
        let nid1 = NodeId::from_hexdigest(
            "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7c")
            .unwrap();
        let nid2 = NodeId::from_hexdigest(
            "dd3311cda63ca68104bcd392bfa0e26d2f911b6f7ab20f505ab9636fe2094e3f")
            .unwrap();
        let nid3 = NodeId::from_hexdigest(
            "d7ea74aad5a4d4523f4b08ae90f5e9d83ec590b319b24e6c8ddbfb305c4b5000")
            .unwrap();
        let mut nids = vec![nid1, nid2];
        nids.sort_by(|a, b| nid3.distance_cmp(a, b));
        assert_eq!(nids, vec![nid2, nid1]);
    }

    #[test]
    fn test_safe_parsing() {
        let nid = NodeId::from_hexdigest("♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥!");
        assert!(nid.is_none());
    }

    #[test]
    fn test_log_distance_0() {
        let nid1 = NodeId::from_hexdigest(
            "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7c")
            .unwrap();
        let nid2 = NodeId::from_hexdigest(
            "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7d")
            .unwrap();
        assert_eq!(nid1.log_distance(&nid2), 0);
    }

    #[test]
    fn test_log_distance_79() {
        let nid1 = NodeId::from_hexdigest(
            "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7c")
            .unwrap();
        let nid2 = NodeId::from_hexdigest(
            "316b370b13056e7358bb33aa85a114471832b295dcc500000000000000000000")
            .unwrap();
        assert_eq!(nid1.log_distance(&nid2), 79);
    }
}
