use std::collections::HashMap;
use vstd::prelude::*;

verus! {

fn mv(old_name: String, new_name: String, fs: &mut HashMap<String, Vec<u8>>) -> Result<(), std::io::Error>
    requires
        !old_name.contains('/') && !old_name.contains('\\'),
        !new_name.contains('/') && !new_name.contains('\\'),
    ensures
        match result {
            Ok(()) => {
                // File was successfully moved
                old(&*fs).contains_key(&old_name) ==> {
                    fs.contains_key(&new_name) && 
                    fs[&new_name] == old(&*fs)[&old_name] &&
                    !fs.contains_key(&old_name) &&
                    forall|k: String| k != old_name && k != new_name ==> 
                        fs.contains_key(&k) == old(&*fs).contains_key(&k) &&
                        (fs.contains_key(&k) ==> fs[&k] == old(&*fs)[&k])
                }
            },
            Err(_) => {
                // On error, filesystem model remains unchanged
                *fs == old(&*fs)
            }
        }
{
    // First check if old file exists in our model
    if !fs.contains_key(&old_name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Source file not found"
        ));
    }

    // Perform the actual filesystem operation
    match std::fs::rename(&old_name, &new_name) {
        Ok(()) => {
            // Update our model to reflect the successful move
            let file_contents = fs.remove(&old_name).unwrap();
            fs.insert(new_name, file_contents);
            Ok(())
        },
        Err(e) => {
            // Filesystem operation failed, model remains unchanged
            Err(e)
        }
    }
}

} // verus!

fn main() {
    println!("Hello, world!");
}
