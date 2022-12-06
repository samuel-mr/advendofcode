#![allow(dead_code)]

#[allow(non_snake_case)]
#[derive(PartialEq, Debug)]
struct DataAction {
    Move: i32,
    From: usize,
    To: usize,
}

#[test]
fn move_one_test() {
    let mut init: Vec<Vec<char>> = vec![vec!['A'], vec!['X', 'Y', 'B']];
    let result: Vec<Vec<char>> = vec![vec!['A', 'B'], vec!['X', 'Y']];

    apply_action_to_array(
        &mut init,
        DataAction {
            Move: 1, // 1 movimiento
            From: 2, // segunda columna
            To: 1,   // hacia la primera columna
        },
    );

    assert_eq!(result, init);
}

#[test]
fn move_two_test() {
    let mut init: Vec<Vec<char>> = vec![vec!['A'], vec!['X', 'Y', 'B']];
    let result: Vec<Vec<char>> = vec![vec!['A', 'B', 'Y', 'X'], vec![]];

    apply_action_to_array(
        &mut init,
        DataAction {
            Move: 3, // 2 movimientos
            From: 2, // segunda columna
            To: 1,   // hacia la primera columna
        },
    );

    assert_eq!(result, init);
}

fn apply_action_to_array(arr: &mut Vec<Vec<char>>, action: DataAction) {
    for _ in 0..action.Move {
        let origin = arr
            .get_mut(action.From - 1)
            .expect("No se encontró  {action.From}");
        let last_value = origin.pop().unwrap();

        let destination = arr
            .get_mut(action.To - 1)
            .expect("No se encontró {action.To}");
        destination.push(last_value);
    }
}

fn get_top_characters(arr: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for item in arr {
        if let Some(value) = item.last() {
            result.push(*value);
        } else {
            println!("No hay valores en este array");
        }
    }
    return result;
}

#[test]
fn get_top_characters_test() {
    assert_eq!(
        "AB",
        get_top_characters(&vec![vec!['A'], vec!['X', 'Y', 'B']])
    );
    assert_eq!(
        "123",
        get_top_characters(&vec![vec!['1'], vec!['X', 'Y', '2'], vec!['X', '3']])
    );
    assert_eq!("1", get_top_characters(&vec![vec!['1'], vec![], vec![]]));
}

#[test]
fn test() {
    assert_eq!(
        DataAction {
            Move: 1,
            From: 9,
            To: 2,
        },
        transform_to_dataaction("move 1 from 9 to 2")
    );
}

// move 1 from 9 to 2
fn transform_to_dataaction(text: &str) -> DataAction {
    let (mut moves, mut from, mut to) = (0, 0, 0);
    let arr = text.split_whitespace();
    for (i, item) in arr.enumerate() {
        if i == 1 {
            moves = item.parse::<i32>().unwrap();
        } else if i == 3 {
            from = item.parse::<usize>().unwrap();
        } else if i == 5 {
            to = item.parse::<usize>().unwrap();
        }
    }
    return DataAction {
        Move: moves,
        From: from,
        To: to,
    };
}

pub fn run() {
    /* graph:
    [V]     [B]                     [C]
    [C]     [N] [G]         [W]     [P]
    [W]     [C] [Q] [S]     [C]     [M]
    [L]     [W] [B] [Z]     [F] [S] [V]
    [R]     [G] [H] [F] [P] [V] [M] [T]
    [M] [L] [R] [D] [L] [N] [P] [D] [W]
    [F] [Q] [S] [C] [G] [G] [Z] [P] [N]
    [Q] [D] [P] [L] [V] [D] [D] [C] [Z]
     1   2   3   4   5   6   7   8   9
        */

    let mut graph: Vec<Vec<char>> = vec![
        vec!['Q', 'F', 'M', 'R', 'L', 'W', 'C', 'V'],
        vec!['D', 'Q', 'L'],
        vec!['P', 'S', 'R', 'G', 'W', 'C', 'N', 'B'],
        vec!['L', 'C', 'D', 'H', 'B', 'Q', 'G'],
        vec!['V', 'G', 'L', 'F', 'Z', 'S'],
        vec!['D', 'G', 'N', 'P'],
        vec!['D', 'Z', 'P', 'V', 'F', 'C', 'W'],
        vec!['C', 'P', 'D', 'M', 'S'],
        vec!['Z', 'N', 'W', 'T', 'V', 'M', 'P', 'C'],
    ];

    let file = crate::shared::get_file_as_string("assets/day5_input.txt");
    for (i, line) in file.lines().enumerate() {
        if i < 10 {
            continue;
        }

        let dataaction = transform_to_dataaction(line);
        apply_action_to_array(&mut graph, dataaction);
    }
    
    let result = get_top_characters(&graph);
    println!("result: {:?}", result);
}
