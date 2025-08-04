use std::collections::HashMap;
use vstd::prelude::*;
use crate::lib::*;

verus! {

pub fn swap(file1: &str, file2: &str, fs: &mut HashMap<String, Vec<u8>>)
    ensures
        // If both files existed before, they are swapped
        (get_file(&old(fs), file1).is_some() && get_file(&old(fs), file2).is_some()) ==> (
            get_file(fs, file1) == get_file(&old(fs), file2) &&
            get_file(fs, file2) == get_file(&old(fs), file1) &&
            forall|k: &str| k != file1 && k != file2 ==>
                get_file(fs, k) == get_file(&old(fs), k)
        ),
        // If only file1 existed, it becomes file2 and file1 doesn't exist
        (get_file(&old(fs), file1).is_some() && get_file(&old(fs), file2).is_none()) ==> (
            get_file(fs, file2) == get_file(&old(fs), file1) &&
            get_file(fs, file1).is_none() &&
            forall|k: &str| k != file1 && k != file2 ==>
                get_file(fs, k) == get_file(&old(fs), k)
        ),
        // If only file2 existed, it becomes file1 and file2 doesn't exist
        (get_file(&old(fs), file1).is_none() && get_file(&old(fs), file2).is_some()) ==> (
            get_file(fs, file1) == get_file(&old(fs), file2) &&
            get_file(fs, file2).is_none() &&
            forall|k: &str| k != file1 && k != file2 ==>
                get_file(fs, k) == get_file(&old(fs), k)
        ),
        // If neither file existed, filesystem remains unchanged
        (get_file(&old(fs), file1).is_none() && get_file(&old(fs), file2).is_none()) ==> (
            *fs == old(fs)
        )
{
    let file1_exists = test(file1, fs);
    let file2_exists = test(file2, fs);
    
    if file1_exists && file2_exists {
        // Both files exist - swap them using a temporary name
        let _ = mv(file1, "temp_swap_file", fs);
        let _ = mv(file2, file1, fs);
        let _ = mv("temp_swap_file", file2, fs);
    } else if file1_exists {
        // Only file1 exists - move it to file2
        let _ = mv(file1, file2, fs);
    } else if file2_exists {
        // Only file2 exists - move it to file1
        let _ = mv(file2, file1, fs);
    }
    // If neither exists, do nothing
}

}
