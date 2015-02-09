#![feature(core)]
#![feature(hash)]

mod nodeid;
mod kbucket;
mod util;

use kbucket::KBucket;
use nodeid::NodeId;

fn main (){
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

        println!("initial sighting of nodes 0-4:");
        kb1.walk_list();

        let node1 = NodeId::from_hexdigest("316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf080001").unwrap();
        let loc1 = "localhost:16070".to_string();
        kb1.sighting(node1, loc1);
        println!("after sighting of node 1 again:");
        kb1.walk_list();
}
