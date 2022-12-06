#![allow(dead_code)]

pub fn rock_paper_sissors() {}

#[derive(PartialEq, Debug)]
enum OptionGame {
    Rock,
    Paper,
    Sissor,
}

#[derive(PartialEq, Debug)]
enum ResultGame {
    Draw,
    Win,
    Lose,
}

#[test]
fn compare_hands_test() {
    assert_eq!(
        ResultGame::Win,
        compare_hands(&OptionGame::Rock, &OptionGame::Paper)
    );
    assert_eq!(
        ResultGame::Draw,
        compare_hands(&OptionGame::Paper, &OptionGame::Paper)
    );
    assert_eq!(
        ResultGame::Lose,
        compare_hands(&OptionGame::Sissor, &OptionGame::Paper)
    );
    assert_eq!(
        ResultGame::Draw,
        compare_hands(&OptionGame::Rock, &OptionGame::Rock)
    );
    assert_eq!(
        ResultGame::Win,
        compare_hands(&OptionGame::Sissor, &OptionGame::Rock)
    );
    assert_eq!(
        ResultGame::Lose,
        compare_hands(&OptionGame::Paper, &OptionGame::Rock)
    );
    assert_eq!(
        ResultGame::Draw,
        compare_hands(&OptionGame::Sissor, &OptionGame::Sissor)
    );
    assert_eq!(
        ResultGame::Win,
        compare_hands(&OptionGame::Paper, &OptionGame::Sissor)
    );
    assert_eq!(
        ResultGame::Lose,
        compare_hands(&OptionGame::Rock, &OptionGame::Sissor)
    );
}

fn compare_hands(external: &OptionGame, yo: &OptionGame) -> ResultGame {
    match external {
        OptionGame::Rock => match yo {
            OptionGame::Rock => ResultGame::Draw,
            OptionGame::Paper => ResultGame::Win,
            OptionGame::Sissor => ResultGame::Lose,
        },
        OptionGame::Paper => match yo {
            OptionGame::Rock => ResultGame::Lose,
            OptionGame::Paper => ResultGame::Draw,
            OptionGame::Sissor => ResultGame::Win,
        },
        OptionGame::Sissor => match yo {
            OptionGame::Rock => ResultGame::Win,
            OptionGame::Paper => ResultGame::Lose,
            OptionGame::Sissor => ResultGame::Draw,
        },
    }
}

fn get_option_score(yo: &OptionGame) -> i32 {
    match yo {
        OptionGame::Rock => 1,
        OptionGame::Paper => 2,
        OptionGame::Sissor => 3,
    }
}

fn get_match_score(result: &ResultGame) -> i32 {
    match result {
        ResultGame::Draw => 3,
        ResultGame::Win => 6,
        ResultGame::Lose => 0,
    }
}

#[test]
fn total_test() {
    assert_eq!(9, total_score(&OptionGame::Sissor, &ResultGame::Win));
    assert_eq!(6, total_score(&OptionGame::Sissor, &ResultGame::Draw));
    assert_eq!(8, total_score(&OptionGame::Paper, &ResultGame::Win));
    assert_eq!(7, total_score(&OptionGame::Rock, &ResultGame::Win));
}

fn total_score(mi_option: &OptionGame, game: &ResultGame) -> i32 {
    let value_score_yo = get_option_score(&mi_option);
    let value_match_yo = get_match_score(&game);

    //println!("from option:{value_score_yo}. match:{value_match_yo}");
    return value_score_yo + value_match_yo;
}

fn play(tu: &OptionGame, mi_option: &OptionGame) -> i32 {
    let my_result = compare_hands(tu, mi_option);
    let total_score = total_score(mi_option, &my_result);
    return total_score;
}

#[test]
fn play_test() {
    assert_eq!(9, play(&OptionGame::Paper, &OptionGame::Sissor));
    assert_eq!(8, play(&OptionGame::Rock, &OptionGame::Paper));
    assert_eq!(1, play(&OptionGame::Paper, &OptionGame::Rock));
}

fn convert_text_to_option(text: &str) -> OptionGame {
    match text {
        "A" => OptionGame::Rock,
        "B" => OptionGame::Paper,
        "C" => OptionGame::Sissor,
        "X" => OptionGame::Rock,
        "Y" => OptionGame::Paper,
        "Z" => OptionGame::Sissor,
        _ => panic!("not implemented"),
    }
}

fn convert_text_to_result(text: &str) -> ResultGame {
    match text {
        "X" => ResultGame::Lose,
        "Y" => ResultGame::Draw,
        "Z" => ResultGame::Win,
        _ => panic!("not implemented"),
    }
}

// read the file with the played game (oponent vs me). I need to sum all scores of every single match
pub fn left_and_rith_are_played_games() {
    let mut total = 0;

    let file = crate::shared::get_file_as_string("assets/day2_input.txt");
    for line in file.lines() {
        let mut lista = line.split_whitespace();
        let uno = lista.next().unwrap();
        let dos = lista.next().unwrap();

        let tu = convert_text_to_option(uno);
        let yo = convert_text_to_option(dos);

        let score = play(&tu, &yo);
        total += score;
    }
    println!("total: {total}");
}

#[test]
fn what_needed_to_win_test(){
    assert_eq!(OptionGame::Sissor , what_needed_to_win(&OptionGame::Paper,   &ResultGame::Win));
    assert_eq!(OptionGame::Paper , what_needed_to_win(&OptionGame::Rock,   &ResultGame::Win));
    assert_eq!(OptionGame::Rock , what_needed_to_win(&OptionGame::Sissor,   &ResultGame::Win));
}

fn what_needed_to_win(move_oponent: &OptionGame, expected: &ResultGame) -> OptionGame {
    match expected {
        ResultGame::Draw => match move_oponent {
            OptionGame::Rock => OptionGame::Rock,
            OptionGame::Paper => OptionGame::Paper,
            OptionGame::Sissor => OptionGame::Sissor,
        },
        ResultGame::Win => match  move_oponent {
            OptionGame::Rock => OptionGame::Paper,
            OptionGame::Paper => OptionGame::Sissor,
            OptionGame::Sissor => OptionGame::Rock,
        },
        ResultGame::Lose => match  move_oponent {
            OptionGame::Rock => OptionGame::Sissor,
            OptionGame::Paper => OptionGame::Rock,
            OptionGame::Sissor => OptionGame::Paper,
        },
    }
}

// read the file and for each line:
// left column = opponent's move
// right column = desired match
pub fn with_expected_match() {
    let mut total = 0;

    let file = crate::shared::get_file_as_string("assets/day2_input.txt");
    for line in file.lines() {
        let mut lista = line.split_whitespace();
        let uno = lista.next().unwrap();
        let dos = lista.next().unwrap();

        let tu = convert_text_to_option(uno);
        let suggested_result = convert_text_to_result(dos);
       let my_suggested_move  = what_needed_to_win(&tu,&suggested_result);

        let score = play(&tu, &my_suggested_move);
        total += score;
    }
    println!("total: {total}");
}