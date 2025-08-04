use std::collections::HashMap;
use vstd::prelude::*;
use crate::lib::*;

verus! {

pub fn swap(file1: &str, file2: &str, fs: &mut HashMap<String, Vec<u8>>) -> (result: Result<(), MvError>)
    requires
        file1 != file2,
        file2 != "tmp_file",
        file1 != "tmp_file",
    ensures
        match result {
            Ok(()) => {
                // If both files existed before, they are swapped
                (get_file(&old(fs), file1).is_some() && get_file(&old(fs), file2).is_some()) ==> (
                    get_file(fs, file1) == get_file(&old(fs), file2) &&
                    get_file(fs, file2) == get_file(&old(fs), file1) &&
                    forall|k: &str| k != file1 && k != file2 && k != "tmp_file" ==>
                        get_file(fs, k) == get_file(&old(fs), k)
                ) &&
                // Otherwise, filesystem remains unchanged
                (get_file(&old(fs), file1).is_none() || get_file(&old(fs), file2).is_none()) ==> (
                    *fs == old(fs)
                )
            },
            Err(MvError) => {
                    forall|k: &str| k != file1 && k != file2 && k != "tmp_file" ==>
                        get_file(fs, k) == get_file(&old(fs), k)
            }
        }
{
    let file1_exists = test(file1, fs);
    let file2_exists = test(file2, fs);
    
    if file1_exists && file2_exists {
        // Both files exist - swap them using a temporary name
        mv(file1, "tmp_file", fs)?;
        mv(file2, file1, fs)?;
        mv("tmp_file", file2, fs)?;
    }
    // Otherwise, do nothing
    Ok(())
}

}
