use std::collections::HashMap;
use vstd::prelude::*;

verus! {

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct MvFailed;

pub uninterp spec fn get_file(fs: &HashMap<String, Vec<u8>>, filename: &str) -> Option<Vec<u8>>;

pub open spec fn fs_unchanged_except(old_fs: &HashMap<String, Vec<u8>>, new_fs: &HashMap<String, Vec<u8>>, changed_files: Seq<&str>) -> bool {
    forall|k: &str| 
        (get_file(new_fs, k) != get_file(old_fs, k)) ==> 
        changed_files.contains(k)
}

#[verifier::external_body]
pub fn mv(old_name: &str, new_name: &str, fs: &mut HashMap<String, Vec<u8>>) -> (result: Result<(), MvFailed>)
    requires get_file(&old(fs), old_name).is_some()
    ensures
        match result {
            Ok(()) => {
                if old_name == new_name {
                    // Moving to same location is a no-op
                    *fs == old(fs)
                } else {
                    get_file(fs, new_name) == get_file(&old(fs), old_name) &&
                    get_file(fs, old_name).is_none() &&
                    forall|k: &str| k != old_name && k != new_name ==>
                        get_file(fs, k) == get_file(&old(fs), k)
                }
            },
            Err(MvFailed) => {
                *fs == old(fs)
            }
        }
{
    std::fs::rename(&old_name, &new_name).map_err(|_| MvFailed)
}

#[verifier::external_body]
pub fn test(filename: &str, fs: &HashMap<String, Vec<u8>>) -> (result: bool)
    ensures
        result == get_file(fs, filename).is_some()
{
    std::path::Path::new(filename).exists()
}

}
