use std::collections::HashMap;
use vstd::prelude::*;

mod lib;
use lib::*;

verus! {

fn main() {
    let mut fs = HashMap::new();
    
    if test("foo", &fs) {
        mv("foo", "bar", &mut fs);
    }
}

} // verus!
