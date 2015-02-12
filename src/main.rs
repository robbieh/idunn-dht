#![feature(core)]
#![feature(hash)]

mod nodeid;
mod kbucket;
mod routingtable;
mod util;

use kbucket::KBucket;
use nodeid::NodeId;
use routingtable::RoutingTable;

fn main (){
    let me = NodeId::from_hexdigest(
        "316b370b13056e7358bb33aa85a114471832b295dcc5888b6785697bcf08ad7c").unwrap();

    let mut rt = RoutingTable::new(me);

    let n1 = NodeId::from_hexdigest(
        "316b370000000000000000000000000000000000000000000000000000000000").unwrap();
   
    rt.sighting(&n1, "localhost:0".to_string());
    println!("{:?}", rt);
}
