use vstd::prelude::*;

verus! {

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct OperationFailed;

pub struct FileSystem;

pub uninterp spec fn get_file(fs: &FileSystem, filename: &str) -> Option<Vec<u8>>;

pub open spec fn unchanged_except(old_fs: &FileSystem, new_fs: &FileSystem, changed_files: Seq<&str>) -> bool {
    forall|k: &str|
        (get_file(new_fs, k) != get_file(old_fs, k)) ==>
        changed_files.contains(k)
}

#[verifier::external_body]
pub fn str_equal(s1: &str, s2: &str) -> (result: bool)
    ensures result == (s1 == s2)
{
    s1 == s2
}

#[verifier::external_body]
pub fn mv(old_name: &str, new_name: &str, fs: &mut FileSystem) -> (result: Result<(), OperationFailed>)
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
                    unchanged_except(&old(fs), fs, seq![old_name, new_name])
                }
            },
            Err(OperationFailed) => {
                *fs == old(fs)
            }
        }
{
    std::fs::rename(&old_name, &new_name).map_err(|_| OperationFailed)
}

#[verifier::external_body]
pub fn cp(src: &str, dst: &str, fs: &mut FileSystem) -> (result: Result<(), OperationFailed>)
    requires get_file(&old(fs), src).is_some(),
             src != dst
    ensures
        match result {
            Ok(()) => {
                get_file(fs, dst) == get_file(&old(fs), src) &&
                get_file(fs, src) == get_file(&old(fs), src) &&
                unchanged_except(&old(fs), fs, seq![dst])
            },
            Err(OperationFailed) => {
                *fs == old(fs)
            }
        }
{
    std::fs::copy(src, dst).map(|_| ()).map_err(|_| OperationFailed)
}

#[verifier::external_body]
pub fn rm(filename: &str, fs: &mut FileSystem) -> (result: Result<(), OperationFailed>)
    requires get_file(&old(fs), filename).is_some()
    ensures
        match result {
            Ok(()) => {
                get_file(fs, filename).is_none() &&
                unchanged_except(&old(fs), fs, seq![filename])
            },
            Err(OperationFailed) => {
                *fs == old(fs)
            }
        }
{
    std::fs::remove_file(filename).map_err(|_| OperationFailed)
}

#[verifier::external_body]
pub fn test(filename: &str, fs: &FileSystem) -> (result: bool)
    ensures
        result == get_file(fs, filename).is_some()
{
    std::path::Path::new(filename).exists()
}

}
