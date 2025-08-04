use std::collections::HashMap;
use crate::lib::*;
use crate::swap_spec::*;
use vstd::prelude::*;

verus! {


pub fn swap(file1: &str, file2: &str, fs: &mut HashMap<String, Vec<u8>>) -> (result: Result<(), SwapError>)
    ensures
        swap_is_correct(file1, file2, &old(fs), fs, result)
{
    if str_equal(file1, file2) || str_equal(file1, "tmp_file") || str_equal(file2, "tmp_file") {
        return Err(SwapError::BadArgs);
    }

    let file1_exists = test(file1, fs);
    let file2_exists = test(file2, fs);

    // Uncomment this and verification will fail
    // if ! str_equal("important_file", file1) && ! str_equal("important_file", file2) && test("important_file", fs) {
    //     rm("important_file", fs)?;
    // }

    if ! (file1_exists && file2_exists) {
        return Err(SwapError::BadArgs)
    }
    match mv(file1, "tmp_file", fs) {
        Ok(()) => {},
        Err(e) => return Err(From::from(e)),
    }
    match mv(file2, file1, fs) {
        Ok(()) => {},
        Err(e) => return Err(From::from(e)),
    }
    match mv("tmp_file", file2, fs) {
        Ok(()) => {},
        Err(e) => return Err(From::from(e)),
    }
    Ok(())
}

}
