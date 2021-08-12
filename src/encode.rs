pub fn base32_encode(target: i32) -> String {
    let alphabet: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a',
                                    'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
                                    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v'];

    let mut number = target.abs();
    let mut final_data = String::new();
    let mut i: i32;
    while number != 0 {
        i = number % 32;
        number = (number - 1) / 32;
        final_data = format!("{}{}", alphabet[i as usize].to_string(), final_data);
    }
    if target < 0 {
        final_data = format!("{}{}", "-", final_data);
    }

    if final_data != "".to_string() {
        return final_data;
    } else {
        return alphabet[0 as usize].to_string();
    }
}

