

fn main() {
    println!();
    let text = String::from("hello first \" and hello the second\"");
    println!("the text is: '{text}'");

    let first_quote: usize = text.find('"').unwrap(); // cuz its an option
    println!("index of first quote (\"): {}", first_quote);

    // but how do i get the second quote ?
    // apply find over slice
                            // from second quote + 1 to the end
    let second_quote: usize = text[first_quote + 1..]
        .find('"')
        .map(|i| i + first_quote + 1)
        .unwrap();
    println!("index of second quote (\"): {}", second_quote);

    // what happened?
    // here text[first_quote + 1..] you get a slice
    // but that slice starts from index = 0 from its perspective
    // but in your perspective ( the developer ) that slice is from the original text
    // well it doesnt think exactly like you
    // without map you are getting a slice ` and hello the second"`
    // which is of length of 21 characters
    // but the index 21 inside our original string
    // is just right here ` and hel|` <- right after hel
                                            // and find again just like before
    println!("look, text from 0 to 21 (text[0..21]) is: '{}'", &text[0..21]);

    // but what if i want to extract the text between quotes ?
    let text_between_quotes: &str = &text[first_quote + 1..second_quote];
    println!("text between quotes: '{}'", text_between_quotes);
    println!();
}