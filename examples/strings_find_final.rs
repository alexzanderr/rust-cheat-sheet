

fn find_from<'a>(string_slice: &str, to_find: &str, start_index: usize) -> usize {
    string_slice[start_index..]
        .find(to_find) // find the pattern
        .map(|index| index + start_index) // add the start index to the found one
        .unwrap()
}

fn main() {
    let string_with_17_chars = String::from("me and me and you");
    let index = find_from(&string_with_17_chars, "me", 0);
    // index: 0, thats right its just at the beginning
    println!("index: {index}");
    println!("text from index: '{}'", &string_with_17_chars[index..]);


    let index = find_from(&string_with_17_chars, "me", 5); // from 5
    println!("index: {index}");
    // the second me its located on index 7
    println!("text from index: '{}'", &string_with_17_chars[index..]);

    // the function its working

}