use vstd::prelude::*;
use crate::lib::*;

verus! {

#[derive(PartialEq, Eq)]
pub enum SwapError {
    BadArgs,
    OperationFailed,
}

} // verus!

impl From<OperationFailed> for SwapError {
    fn from(_: OperationFailed) -> Self {
        SwapError::OperationFailed
    }
}

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
                get_file(fs, "tmp_file").is_none() &&
                unchanged_except(old_fs, fs, seq![file1, file2, "tmp_file"])
            )
        },
        Err(SwapError::BadArgs) => {
            *fs == *old_fs
        },
        Err(SwapError::OperationFailed) => {
            unchanged_except(old_fs, fs, seq![file1, file2, "tmp_file"])
        }
    }
}

}
