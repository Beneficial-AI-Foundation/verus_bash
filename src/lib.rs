use std::collections::HashMap;
use vstd::prelude::*;

verus! {

#[derive(PartialEq, Eq)]
pub struct MvError;

pub uninterp spec fn get_file(fs: &HashMap<String, Vec<u8>>, filename: &str) -> Option<Vec<u8>>;

#[verifier::external_body]
pub fn mv(old_name: &str, new_name: &str, fs: &mut HashMap<String, Vec<u8>>) -> (result: Result<(), MvError>)
    requires get_file(&old(fs), old_name).is_some()
    ensures
        match result {
            Ok(()) => {
                    get_file(fs, new_name) == get_file(&old(fs), old_name) &&
                    get_file(fs, old_name).is_none() &&
                    forall|k: &str| k != old_name && k != new_name ==>
                        get_file(fs, k) == get_file(&old(fs), k)
            },
            Err(MvError) => {
                *fs == old(fs)
            }
        }
{
    std::fs::rename(&old_name, &new_name).map_err(|_| MvError)
}

#[verifier::external_body]
pub fn test(filename: &str, fs: &HashMap<String, Vec<u8>>) -> (result: bool)
    ensures
        result == get_file(fs, filename).is_some()
{
    std::path::Path::new(filename).exists()
}

}