use vstd::prelude::*;
use crate::lib::*;
use crate::swap1::SwapError;

verus! {

pub open spec fn swap_is_correct(
    file1: &str, 
    file2: &str, 
    old_fs: &FileSystem, 
    fs: &FileSystem, 
    result: Result<(), SwapError>
) -> bool {
    match result {
        Ok(()) => {
            (
                get_file(fs, file1) == get_file(old_fs, file2) &&
                get_file(fs, file2) == get_file(old_fs, file1) &&
                unchanged_except(old_fs, fs, seq![file1, file2, "tmp_file"])
            )
        },
        Err(SwapError::BadArgs) => {
            *fs == *old_fs
        },
        Err(SwapError::MvFailed) => {
            unchanged_except(old_fs, fs, seq![file1, file2, "tmp_file"])
        }
    }
}

}