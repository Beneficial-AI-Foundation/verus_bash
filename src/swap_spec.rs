use crate::lib::*;
use vstd::prelude::*;

verus! {

#[derive(PartialEq, Eq)]
pub enum SwapError {
    BadArgs,
    OperationFailed,
}

impl From<OperationFailed> for SwapError {
    fn from(_x: OperationFailed) -> (res: Self)
        ensures res == SwapError::OperationFailed
    {
        SwapError::OperationFailed
    }
}

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
