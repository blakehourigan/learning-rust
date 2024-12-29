fn main() {
    let s = String::from("ay bruh watch yo jet");

    let pig_latin_sentence = convert_to_pig_latin(s);
    println!("{pig_latin_sentence}");
}

fn convert_to_pig_latin(some_string: String) -> String {
    let mut pig_latin;

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut strings: Vec<String> = vec![];

    for word in some_string.split(' ') {
        let mut chars: Vec<u8> = word.as_bytes().to_vec();

        let first_char = chars[0] as char;

        if vowels.contains(&first_char) {
            pig_latin = format!("{word}-hay");
        } else {
            chars.remove(0);
            let new_word: String =
                String::from_utf8(chars).expect("Expected byte represented chars as input");

            pig_latin = format!("{new_word}-{}ay", first_char);
        }
        strings.push(pig_latin.clone());
    }
    let pig_latin_sentence = strings.join(" ");

    pig_latin_sentence
}
