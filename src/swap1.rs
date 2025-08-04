use crate::lib::*;
use crate::swap_spec::*;
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
        swap_is_correct(file1, file2, &old(fs), fs, result)
{
    if str_equal(file1, file2) || str_equal(file1, "tmp_file") || str_equal(file2, "tmp_file") {
        return Err(SwapError::BadArgs);
    }

    let file1_exists = test(file1, fs);
    let file2_exists = test(file2, fs);

    if ! (file1_exists && file2_exists) {
        return Err(SwapError::BadArgs)
    }
    mv(file1, "tmp_file", fs).map_err(|x| SwapError::MvFailed)?;
    mv(file2, file1, fs).map_err(|x| SwapError::MvFailed)?;
    mv("tmp_file", file2, fs).map_err(|x| SwapError::MvFailed)?;
    Ok(())
}

}
