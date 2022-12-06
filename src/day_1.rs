use itertools::Itertools;

#[allow(dead_code)]
// Nota: cada espacio en el archivo significa que es otro grupo diferente
// obtener los 3 primeros grupos que sumen más dentro de la lista de numeros
pub fn get_sum_top_3() {
    let file = crate::shared::get_file_as_string("assets/input.txt");

    let mut sumas: Vec<i32> = Vec::new();
    let filas = file.lines().collect::<Vec<&str>>();
    for (_, b) in filas.into_iter().group_by(|e| !(*e).is_empty()).into_iter() {
        let sum: i32 = b.map(|x| x.parse::<i32>().unwrap_or(0)).sum();

        sumas.push(sum);
        //println!("a: {a} sum: {sum}");
    }

    sumas.sort_by(|a, b| b.cmp(a)); // descending
    let length = sumas.len();
    println!("{length}");
    let result: i32 = sumas.iter().take(3).sum();

    println!("{result}");
}

#[allow(dead_code)]
// Obtener que grupo tiene el mayor X (X= suma de cada numero interno)
fn get_maximun() {
    let file = crate::shared::get_file_as_string("assets/input.txt");

    let mut max = 0;

    let mut accumulated = 0;

    for line in file.lines() {
        if line.is_empty() {
            //println!("{accumulated}");
            max = max.max(accumulated);
            accumulated = 0;
        } else {
            let num: i32 = line.parse().unwrap();
            accumulated += i32::from(num);
        }
    }
    println!("max: {max}  ");
}

// fuerza bruta
