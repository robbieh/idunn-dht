use std::cmp::Ordering;
use std::num;

const NODE_ID_SIZE: usize = 32;

#[derive(Show, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId([u8; NODE_ID_SIZE]);

impl NodeId {
    #[allow(unstable)]
    pub fn from_hexdigest(hex: &str) -> Option<NodeId> {
        if hex.len() != 2 * NODE_ID_SIZE { return None; }
        let mut result = [0u8; NODE_ID_SIZE];
        for i in (0..NODE_ID_SIZE) {
            let i2 = i * 2;
            let digit = &hex[i2..(i2 + 2)];
            match num::from_str_radix::<u8>(digit, 16) {
                None => { return None; },
                Some(d) => { result[i] = d; }
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
}

#[cfg(test)]
mod tests {
    use super::{NodeId};

    #[test]
    fn test_from_hexdigest() {
        let nid = NodeId::from_hexdigest(
            "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7c");
        assert!(nid.is_some());
        let nid = nid.unwrap();
        assert_eq!(nid.0[0], 0x31u8);
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
}
