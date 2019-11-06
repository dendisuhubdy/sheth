use arrayref::array_ref;
use sheth::process_data_blob;
use std::fs;

/// The `main` function is the entry point when `sheth` is compiled as an executable binary.
/// Testing `sheth` in this manner is usually better than running through `Scout`, because your
/// local system will print coherent stack traces where as the WebAssembly interpreter will just
/// notify that a `Trap` has been detected.
pub fn main() {
    let input = fs::read_to_string("blob").expect("File to exist");

    let args: Vec<&str> = input.split_whitespace().collect();
    let pre_state_root = hex::decode(args[0]).unwrap();
    let post_state_root = hex::decode(args[1]).unwrap();
    let mut input = hex::decode(args[2]).unwrap();

    // Process input data
    let post_root = process_data_blob(&mut input, array_ref![pre_state_root, 0, 32]);

    assert_eq!(post_root, *array_ref![post_state_root, 0, 32]);

    println!("pre_state_root  => {:?}", hex::encode(pre_state_root));
    println!("post_state_root => {:?}", hex::encode(post_root));
}
