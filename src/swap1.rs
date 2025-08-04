use crate::lib::*;
use vstd::prelude::*;

verus! {

// Wrapper function for string equality
#[verifier::external_body]
fn str_equal(s1: &str, s2: &str) -> (result: bool)
    ensures result == (s1 == s2)
{
    s1 == s2
}

#[derive(PartialEq, Eq)]
pub enum SwapError {
    BadArgs,
    MvFailed,
}

pub fn swap(file1: &str, file2: &str, fs: &mut FileSystem) -> (result: Result<(), SwapError>)
    ensures
        match result {
            Ok(()) => {
                (
                    get_file(fs, file1) == get_file(&old(fs), file2) &&
                    get_file(fs, file2) == get_file(&old(fs), file1) &&
                    unchanged_except(&old(fs), fs, seq![file1, file2, "tmp_file"])
                )
            },
            Err(SwapError::BadArgs) => {
                *fs == old(fs)
            },
            Err(SwapError::MvFailed) => {
                    unchanged_except(&old(fs), fs, seq![file1, file2, "tmp_file"])
            }
        }
{
    // Check for bad arguments
    if str_equal(file1, file2) || str_equal(file1, "tmp_file") || str_equal(file2, "tmp_file") {
        return Err(SwapError::BadArgs);
    }

    let file1_exists = test(file1, fs);
    let file2_exists = test(file2, fs);

    if ! (file1_exists && file2_exists) {
        return Err(SwapError::BadArgs)
    }
    mv(file1, "tmp_file", fs).map_err(|x| SwapError::MvFailed)?;
    match mv(file2, file1, fs) {
        Ok(()) => {},
        Err(MvFailed) => return Err(SwapError::MvFailed),
    }
    match mv("tmp_file", file2, fs) {
        Ok(()) => {},
        Err(MvFailed) => return Err(SwapError::MvFailed),
    }
    Ok(())
}

}
