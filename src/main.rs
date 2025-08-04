use std::collections::HashMap;
use vstd::prelude::*;

mod lib;

verus! {

fn main() {
    let mut fs = HashMap::new();
    
    if lib::test("foo", &fs) {
        lib::mv("foo", "bar", &mut fs);
    }
}

} // verus!
