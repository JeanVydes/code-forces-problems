fn solve<R: BufRead, W: Write>(mut input: FastInput<R>, mut w: W) {
    let word: Vec<u8> = input.token();
    let to_string = |x: Vec<u8>| String::from_utf8(x).unwrap();
    let word = to_string(word);

    let mut uppercase = 0;
    let mut lowercase = 0;
    for c in word.chars() {
        if c.is_uppercase() {
            uppercase += 1;
        } else {
            lowercase += 1;
        }
    }  

    if uppercase > lowercase {
        for c in word.chars() {
            write!(w, "{}", c.to_uppercase().to_string()).unwrap();
        }
    } else {
        for c in word.chars() {
            write!(w, "{}", c.to_lowercase().to_string()).unwrap();
        }
    }
}
 