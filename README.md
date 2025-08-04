``` bash
mv $1 tmp_file
mv $2 $1
mv tmp_file $2
```

But there are some edge cases this script doesn't handle correctly.

Using the `mv` function defined in this crate, 
we directly to Rust:


``` rust
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
```


Verus gives us an error, since we haven't checked that `file1` exists before we moved it:

``` rust
error: precondition not satisfied
  --> src/swap1.rs:28:11
   |
28 |     match mv(file1, "tmp_file", fs) {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
  ::: src/lib.rs:32:14
   |
32 |     requires get_file(&old(fs), old_name).is_some()
   |              -------------------------------------- failed precondition
```

So we add a check beforehand:

``` rust
let file1_exists = test(file1, fs);
let file2_exists = test(file2, fs);
if ! (file1_exists && file2_exists) {
    return Err(SwapError::BadArgs)
}
```

Verus is still unhappy:

``` rust
error: precondition not satisfied
  --> src/swap1.rs:32:11
   |
32 |     match mv(file2, file1, fs) {
   |           ^^^^^^^^^^^^^^^^^^^^
   |
  ::: src/lib.rs:32:14
   |
32 |     requires get_file(&old(fs), old_name).is_some()
   |              -------------------------------------- failed precondition
```
This error is more subtle: If `file1` and `file2` are the same path, 
then `mv(file1, "tmp_file", fs)` will leave nothing at `file2`, 
so `mv(file2, file1, fs)` will fail.

Again we can fix this by adding a check:

``` rust
if str_equal(file1, file2) || str_equal(file1, "tmp_file") || str_equal(file2, "tmp_file") {
    return Err(SwapError::BadArgs);
}
```
(We need those other checks too for verus to be happy.)

Now verus verifies. But we haven't actually proven anything about what this `swap` function does.
We've just shown that it handles edge cases correctly.

Here's a specification for how a `swap` function should change the filesystem:
``` rust
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
```
Importantly, even if the `swap` function fails because of user error or some unexpected
OS error, the spec guarantees that all but three files won't change.
So if we maliciously add `rm("important_file", fs)?;` to `swap`, Verus detects that it now fails the spec.

## Contents

## Limitations
