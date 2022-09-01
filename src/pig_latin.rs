pub fn convert_to_pig_latin(word :&str) -> String {
    let temp_word = word.clone().trim();
    let mut vectorized_word: Vec<char> = Vec::new();
    for letter in temp_word.chars() {
        vectorized_word.push(letter);
    }

    let first_letter = vectorized_word[0];
    let is_vowel_sounded: bool = match first_letter{
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        'h' => true,
        _ => false,
    };

    if is_vowel_sounded == false {
        vectorized_word = vectorized_word[1..].to_vec();
        vectorized_word.push(first_letter);
    } else {
        vectorized_word.push('h');
    }

    vectorized_word.push('a');
    vectorized_word.push('y');

    let mut final_word: String = String::new();

    for element in vectorized_word {
        final_word.push(element);
    }
    dbg!(&final_word);
    final_word
}