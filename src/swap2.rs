use crate::lib::*;
use crate::swap_spec::*;
use std::collections::HashMap;
use vstd::prelude::*;

verus! {

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

    match cp(file1, "tmp_file", fs) {
        Ok(()) => {},
        Err(e) => return Err(From::from(e)),
    }
    match cp(file2, file1, fs) {
        Ok(()) => {},
        Err(e) => return Err(From::from(e)),
    }
    match cp("tmp_file", file2, fs) {
        Ok(()) => {},
        Err(e) => return Err(From::from(e)),
    }
    match rm("tmp_file", fs) {
        Ok(()) => {},
        Err(e) => return Err(From::from(e)),
    }
    Ok(())
}

}
