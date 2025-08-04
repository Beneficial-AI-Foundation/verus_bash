# verus_bash

## Demo

This is a simple bash script that swaps two files:

```bash
mv $1 tmp_file
mv $2 $1
mv tmp_file $2
```

But there are some edge cases this script doesn't handle correctly.

Using the `mv` function defined in this crate,
we translate to Rust:

```rust
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

Verus (via `cargo verus verify`) gives us an error, since we haven't checked that `file1` exists before we moved it:

```rust
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

```rust
let file1_exists = test(file1, fs);
let file2_exists = test(file2, fs);
if ! (file1_exists && file2_exists) {
    return Err(SwapError::BadArgs)
}
```

Verus is still unhappy:

```rust
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

```rust
if str_equal(file1, file2) || str_equal(file1, "tmp_file") || str_equal(file2, "tmp_file") {
    return Err(SwapError::BadArgs);
}
```

(We need those other checks too for verus to be happy.)

Now verus verifies. But we haven't actually proven anything about what this `swap` function does.
We've just shown that it handles edge cases correctly.

Here's a specification for how a `swap` function should change the filesystem:

```rust
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

Importantly, even if the `swap` function fails because of user error or an
OS error, the spec guarantees that all but three files won't change.
So if we maliciously add `rm("important_file", fs)?;` to `swap`, Verus detects that it now fails the spec.

The `swap` function is executable and can be run with `cargo run`

## Related work

- https://theses.hal.science/tel-03917971v1/file/Va_Jeannerod_Nicolas.pdf
- https://www.irif.fr/~treinen/publi/slides/debconf16.pdf
- https://hal.science/hal-01534747

## Contents

- `main.rs` is just boilerplate to get the user arguments from the command line
- `swap1.rs` contains an implementation of the `swap` function
- `swap2.rs` contains a different implementation of the `swap` function, also proven correct
- `swap_spec.rs` contains the `swap` function's spec
- `lib.rs` tells Verus what `mv`, `cp`, `rm`, and `test` do

## Limitations

- Currently not foolproof against a malicious user
  - A malicious user could create a different `FileSystem` object inside the object and apply operations to it, and the verifier won't notice these operations
  - A malicious user could just call `std::fs::remove_file` directly
  - We could write a macro/a linter script that goes through the function and checks that only approved functions are called
  - We want to give the user enough flexibility to prove that their function is correct, but not enough flexibility to call untrusted Rust code
- The `FileSystem` object has no understanding of directories, permissions, paths, and other basic Unix facts
  - We'd have to tell it how a POSIX file system should work
- Only the easiest shell functions are specified right now
  - e.g. `grep` would need some understanding of regex
  - There's probably no good way to specify something like `curl ... | bash ...`, since that could do anything
    - Even if we can't fully specify that a bash script is correct, we could specify important security properties:
      - No files are created/modified/deleted, except for files/directories allowed in the spec
      - No files are uploaded to the internet, except for ones allowed in the spec
      - No programs are downloaded from the internet and executed, except for URLs allowed in the spec
      - There could be a lemma proving hyperproperties like no leakage of secrets: "if the user's API key were different, the bash script still write the exact same messages to the logfile"
