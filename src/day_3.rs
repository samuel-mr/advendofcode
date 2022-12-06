#![allow(dead_code)]

fn separte_two_groups(line: &str) -> (&str, &str) {
    let num = line.len() / 2;
    let first = &line[0..num];
    let second = &line[num..];

    return (first, second);
}

#[test]
fn separte_two_groups_even_test() {
    let result = separte_two_groups("ab12");
    assert_eq!("ab", result.0);
    assert_eq!("12", result.1);
}

#[test]
fn separte_two_groups_odd_test() {
    let result = separte_two_groups("ab123");
    assert_eq!("ab", result.0);
    assert_eq!("123", result.1);
}

fn get_repited_char(first: &str, second: &str) -> char {
    for i in first.chars() {
        if second.contains(i) {
            return i;
        }
    }
    panic!("No encontró el orror!");
}

#[test]
fn get_repited_char_test() {
    assert_eq!("a", get_repited_char("abc", "1a3").to_string());
    assert_eq!("y", get_repited_char("qwerty", "123y345").to_string());
}

fn get_prioridad(letra: char) -> u32 {
    /*
    a-z  97-122
    A-Z  65-90
    */

    // Valor dados por el ejercicio, representan numéricamente a los rangos de: A-Z y a-z
    const LOWERCASE_INITIAL_POSITION: u32 = 1;
    const UPPERCASE_INITIAL_POSITION: u32 = 27;

    // Valores numericos de cada caracter: a - z
    const A_LOWERCASE_NUMBER: u32 = 97;
    const Z_LOWERCASE_NUMBER: u32 = 122;
    const A_UPPERCASE_NUMBER: u32 = 65;
    const Z_UPPERCASE_NUMBER: u32 = 90;

    //println!("letra 1: {:?}", letra as u32);
    let position_as_number = letra as u32;
    let position = match position_as_number {
        A_LOWERCASE_NUMBER..=Z_LOWERCASE_NUMBER => {
            position_as_number - A_LOWERCASE_NUMBER + LOWERCASE_INITIAL_POSITION
        }
        A_UPPERCASE_NUMBER..=Z_UPPERCASE_NUMBER => {
            position_as_number - A_UPPERCASE_NUMBER + UPPERCASE_INITIAL_POSITION
        }
        _ => panic!("El número {letra} no está permitido"),
    };
    //println!("letra 2: {:?}", position);

    return position;
}

#[test]
fn get_prioridad_test() {
    assert_eq!(1, get_prioridad('a'));
    assert_eq!(26, get_prioridad('z'));
    assert_eq!(27, get_prioridad('A'));
    assert_eq!(52, get_prioridad('Z'));
}

#[test]
fn get_prioridad_from_string_test() {
    assert_eq!(26, get_prioridad_from_string("abz12z"));
    assert_eq!(52, get_prioridad_from_string("abzZ123Z"));
}

fn get_prioridad_from_string(line: &str) -> u32 {
    let (first, second) = separte_two_groups(line);
    return get_prioridad(get_repited_char(first, second));
}

// Leer el archivo
// Cada linea del archivo se debe dividor en 2 grupos (separados exactamente a la mitad)
// Hay un caracter que se repite en los 2 grupos, ese caracter se debe obtener su 'prioridad'
// El total 'sum' es la suma de todas esas 'prioridades'
pub fn exercise_1() {
    let file = crate::shared::get_file_as_string("assets/day3_input.txt");
    let mut sum = 0;
    for line in file.lines() {
        sum += get_prioridad_from_string(line);
    }
    println!("exercise 1: {sum}");
}

// Leer el archivo
// Agrupar cada 3 líneas del archivo
// Buscar la letra que se repite en cada una de las 3 líneas
// A la letra, asignarle su prioridad
// Obtener la sumatoria de todas las prioridades
pub fn exercise_2() {
    let file = crate::shared::get_file_as_string("assets/day3_input.txt");
    let mut sum = 0;
    let mut i = 0;
    const MAX: u32 = 3;

    let mut group: Vec<&str> = Vec::new();
    for line in file.lines() {
        if i <= MAX {
            i +=1;

            group.push(line);

            if i == MAX {
                let repited = get_repited_char_from_array(&group);
                let prioridad = get_prioridad(repited);
                sum += prioridad;
                //reset
                group.clear();
                i = 0;
            }
        }
    }
    println!("exercise 2: {sum}");
}

fn get_repited_char_from_array(group: &Vec<&str>) -> char {
    let mut iterator = group.iter();
    let first = iterator.next().unwrap();
    let second = iterator.next().unwrap();
    let third = iterator.next().unwrap();

    for first_chars in first.chars() {
        if second.contains(first_chars) && third.contains(first_chars) {
            return first_chars;
        }
    }
    panic!("no se encontró un caracter repetido en las 3 líenas");
}

#[test]
fn get_repited_char_from_array_test() {
    assert_eq!('a', get_repited_char_from_array(&vec!["abc", "12a", "tat"]));
    assert_eq!(
        '2',
        get_repited_char_from_array(&vec!["1234", "aaa2bbb", "x299999"])
    );

    assert_eq!(
        'r',
        get_repited_char_from_array(&vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        ])
    );
    assert_eq!(
        'Z',
        get_repited_char_from_array(&vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ])
    );
}
