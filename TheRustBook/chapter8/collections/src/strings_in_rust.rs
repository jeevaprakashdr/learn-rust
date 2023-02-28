use unicode_segmentation::UnicodeSegmentation;
pub fn demo() {
    // Strings are stored as a collection of UTF-8 encoded bytes
    // UTF-8 chars are is of variable length of bytes and we need deeper understanding to process these string
    let hello = String::from("नमस्ते");

    for b in hello.bytes() {
        println!("{}", b);
    }

    for c in hello.chars() {
        println!("{}", c);
    }

    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}