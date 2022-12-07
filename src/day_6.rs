#![allow(dead_code)]

use std::collections::HashMap;

#[test]
fn test() {
    assert_eq!(
        5,
        get_last_position_with_first_x_characters("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)
    );
    assert_eq!(
        6,
        get_last_position_with_first_x_characters("nppdvjthqldpwncqszvftbrmjlhg", 4)
    );
    assert_eq!(
        10,
        get_last_position_with_first_x_characters("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
    );
    assert_eq!(
        11,
        get_last_position_with_first_x_characters("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)
    );
    assert_eq!(
        19,
        get_last_position_with_first_x_characters("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)
    );
    assert_eq!(
        26,
        get_last_position_with_first_x_characters("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)
    );
}

fn get_last_position_with_first_x_characters(text: &str, max_items: usize) -> usize {
    let mut hash: HashMap<char, usize> = HashMap::new();

    for i in 0..text.len() {
        let substring = &text[i..i + max_items];
        for subcharacter in substring.chars() {
            if hash.contains_key(&subcharacter) {
                hash.clear();
                break;
            } else {
                hash.insert(subcharacter, i);
            }
        }
        if hash.len() == max_items {
            return i + max_items;
        }
    }
    return 0;
}

pub fn exercise_1() {
    let text = crate::shared::get_file_as_string("assets/day6_input.txt");
    let result = get_last_position_with_first_x_characters(text.as_str(), 4);
    println!("exercise 1: {result}");
}

pub fn exercise_2() {
    let text = crate::shared::get_file_as_string("assets/day6_input.txt");
    let result = get_last_position_with_first_x_characters(text.as_str(), 14);
    println!("exercise 2: {result}");
}
