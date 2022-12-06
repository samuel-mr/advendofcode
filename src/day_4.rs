#![allow(dead_code)]

#[test]
fn test() {
    assert_eq!(true, is_partial_overlaped_atleast((1, 10), (10, 30)));
    assert_eq!(true, is_partial_overlaped_atleast((1, 10), (2, 3)));
    assert_eq!(true, is_partial_overlaped_atleast((2, 3), (1, 10)));
    assert_eq!(true, is_partial_overlaped_atleast((1, 3), (2, 5)));
    assert_eq!(true, is_partial_overlaped_atleast((1, 3), (3, 5)));

    assert_eq!(true, is_partial_overlaped_atleast((8, 15), (5, 10)));
    assert_eq!(true, is_partial_overlaped_atleast((10, 15), (5, 10)));
    assert_eq!(true, is_partial_overlaped_atleast((2, 5), (1, 3)));
    assert_eq!(true, is_partial_overlaped_atleast((3, 5), (1, 3)));
    
    assert_eq!(false, is_partial_overlaped_atleast((5, 5), (1, 3)));

    assert_eq!(false, is_partial_overlaped_atleast((1, 3), (4, 5)));
    assert_eq!(false, is_partial_overlaped_atleast((3, 3), (4, 4)));
    // 120-121,11-12
    assert_eq!(false, is_partial_overlaped_atleast((40, 41), (1, 2)));
}

fn is_partial_overlaped_atleast(first: (i32, i32), second: (i32, i32)) -> bool {
    // primero me aseguro que se (rango 1 menor al rango mayor)
    if first.0 <= second.1 && second.0 <= first.1 {
        return true;
    } else if second.1 >= first.0 { // me aseguro que 'second' sea 'continacion' de ' first'
        return first.1 >= second.0;
    } else {
        return false;
    }
}

#[test]
fn run_test() {
    assert_eq!(false, is_full_overlaped((2, 4), (6, 8)));
    assert_eq!(false, is_full_overlaped((2, 3), (4, 5)));
    assert_eq!(false, is_full_overlaped((5, 7), (7, 9)));
    assert_eq!(true, is_full_overlaped((2, 8), (3, 7)));
    assert_eq!(true, is_full_overlaped((6, 6), (4, 6)));
    assert_eq!(false, is_full_overlaped((2, 6), (4, 8)));
    assert_eq!(true, is_full_overlaped((82, 82), (8, 83)));
    assert_eq!(false, is_full_overlaped((42, 88), (13, 87)));
    assert_eq!(false, is_full_overlaped((4, 90), (3, 67)));
}

fn is_full_overlaped(arg_1: (i32, i32), arg_2: (i32, i32)) -> bool {
    // verifica que la izquierda contenga a la derecha
    if arg_2.0 >= arg_1.0 && arg_2.1 <= arg_1.1 {
        return true;
    } else {
        // verifica que la derecha contenga a la izquierda
        return arg_2.0 <= arg_1.0 && arg_2.1 >= arg_1.1;
    }
}

fn extract_tuples_i32(text: &str) -> (i32, i32) {
    let mut result = text.split('-');
    let first = result
        .next()
        .unwrap()
        .parse::<i32>()
        .expect("No se puede obtener el primer elemento");
    let second = result
        .next()
        .unwrap()
        .parse::<i32>()
        .expect("No se puede obtener el segundo elemento");
    return (first, second);
    // return (, result.next().unwrap())
}

#[test]
fn extract_tuples_i32_test() {
    assert_eq!((1, 2), extract_tuples_i32("1-2"));
    assert_eq!((29, 115), extract_tuples_i32("29-115"));
}

#[test]
fn extract_tules_str_test() {
    assert_eq!(("a", "b"), extract_tules_str(&"a,b"));
    assert_eq!(("123", "qwe"), extract_tules_str(&"123,qwe"));
}

fn extract_tules_str(text: &str) -> (&str, &str) {
    let mut splited = text.split(',');
    return (splited.next().unwrap(), splited.next().unwrap());
}

#[test]
fn process_test() {
    assert_eq!(false, process_full_overlap("2-4,6-8"));
    assert_eq!(false, process_full_overlap("2-3,4-5"));
    assert_eq!(false, process_full_overlap("5-7,7-9"));
    assert_eq!(true, process_full_overlap("2-8,3-7"));
    assert_eq!(true, process_full_overlap("6-6,4-6"));
    assert_eq!(false, process_full_overlap("2-6,4-8"));

    assert_eq!(true, process_full_overlap("82-82,8-83"));
    assert_eq!(true, process_full_overlap("6-98,6-93"));
    assert_eq!(true, process_full_overlap("56-77,55-82"));
    assert_eq!(true, process_full_overlap("51-68,51-61"));
    assert_eq!(false, process_full_overlap("4-90,3-67"));
    assert_eq!(true, process_full_overlap("29-30,29-97"));
    assert_eq!(false, process_full_overlap("42-88,13-87"));
    assert_eq!(false, process_full_overlap("17-95,33-96"));
    assert_eq!(true, process_full_overlap("11-56,12-56"));
}

fn process_full_overlap(text: &str) -> bool {
    let (pair_a, pair_b) = extract_tules_str(text);
    let first = extract_tuples_i32(&pair_a);
    let second = extract_tuples_i32(&pair_b);

    return is_full_overlaped(first, second);
}

pub fn exercise_1() {
    let text = crate::shared::get_file_as_string("assets/day4_input.txt");
    let mut sum = 0;
    for line in text.lines() {
        let result = process_full_overlap(line);
        if result {
            sum += 1;
        }
    }
    println!("day4 - a :{sum}");
}

// verificar si es parcialmente sobrelapado
pub fn exercise_2() {
    let text = crate::shared::get_file_as_string("assets/day4_input.txt");
    let mut sum = 0;
    for line in text.lines() {
        let result = process_parital_overlap(line);
        if result {
            sum += 1;
        }
    }
    println!("day4 - b :{sum}");
}

#[test]
fn process_parital_overlap_test() {
    assert_eq!(true, process_parital_overlap("82-82,8-83"));
    assert_eq!(true, process_parital_overlap("6-98,6-93"));
    assert_eq!(true, process_parital_overlap("56-77,55-82"));
    assert_eq!(true, process_parital_overlap("51-68,51-61"));
    assert_eq!(true, process_parital_overlap("4-90,3-67"));
    assert_eq!(true, process_parital_overlap("29-30,29-97"));
    assert_eq!(true, process_parital_overlap("42-88,13-87"));
    assert_eq!(true, process_parital_overlap("17-95,33-96"));
    assert_eq!(true, process_parital_overlap("11-56,12-56"));

    assert_eq!(false, process_parital_overlap("11-12,120-121"));
    assert_eq!(false, process_parital_overlap("120-121,11-12"));
}

fn process_parital_overlap(text: &str) -> bool {
    let (pair_a, pair_b) = extract_tules_str(text);
    let first = extract_tuples_i32(&pair_a);
    let second = extract_tuples_i32(&pair_b);

    return is_partial_overlaped_atleast(first, second);
}
