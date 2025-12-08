use std::io::BufRead;

fn resout_maths() -> Result<i128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let lines_vec: Vec<String> = lecteur.lines().map(|l| l.unwrap()).collect();
    let n = lines_vec.len();
    let op: Vec<&str> = lines_vec[n - 1].split_whitespace().collect();
    let n_op = op.len();

    let mut res_vec: Vec<i128> = lines_vec[0]
        .split_whitespace()
        .map(|v| v.parse::<i128>().unwrap())
        .collect();

    for line in lines_vec.iter().take(n - 1).skip(1) {
        let v_vec: Vec<i128> = line
            .split_whitespace()
            .map(|v| v.parse::<i128>().unwrap())
            .collect();

        for j in 0..n_op {
            match op[j] {
                "*" => res_vec[j] *= v_vec[j],
                "+" => res_vec[j] += v_vec[j],
                _ => (),
            }
        }
    }

    Ok(res_vec.iter().sum())
}

fn resout_maths_2() -> Result<u128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let lines_vec: Vec<String> = lecteur.lines().map(|l| l.unwrap()).collect();
    let n = lines_vec.len();

    // On récupère les colonnes strictes avec les opérations (la dernière ligne).
    let op = &lines_vec[n - 1];
    let op_vec = op.chars().fold(Vec::new(), |mut acc, c| {
        let n_acc = acc.len();
        match c {
            '*' => acc.push(vec![c]),
            '+' => acc.push(vec![c]),
            ' ' => acc[n_acc - 1].push(c),
            _ => (),
        };

        acc
    });

    let column_sizes = op_vec.iter().map(|v| v.len()).collect::<Vec<usize>>();
    let nb_op = column_sizes.len();

    let mut strict_columns = vec![Vec::new(); nb_op];
    for line in lines_vec.iter().take(n - 1) {
        let string_vec = line.chars().collect::<Vec<char>>();
        let mut cur = 0;
        for j in 0..nb_op {
            let s = column_sizes[j];
            for idx_c in 0..s {
                strict_columns[j].push(string_vec[cur + idx_c]);
            }

            cur += s;
        }
    }

    // On range les éléments en colonnes.
    let mut strict_columns_formatted: Vec<Vec<Vec<char>>> = vec![Vec::new(); nb_op];
    for i in 0..nb_op {
        strict_columns_formatted[i] = vec![Vec::new(); column_sizes[i]];
    }

    for i in 0..nb_op {
        for j in 0..strict_columns[i].len() {
            strict_columns_formatted[i][j % column_sizes[i]].push(strict_columns[i][j]);
        }
    }

    let mut res = 0;
    for operation_idx in 0..nb_op {
        match op_vec[operation_idx][0] {
            '*' => {
                res += strict_columns_formatted[operation_idx]
                    .iter()
                    .map(|v| {
                        v.iter()
                            .filter(|c| c != &&' ')
                            .collect::<String>()
                            .parse::<u128>()
                            .unwrap_or(1)
                    })
                    .collect::<Vec<u128>>()
                    .iter()
                    .product::<u128>()
            }
            '+' => {
                res += strict_columns_formatted[operation_idx]
                    .iter()
                    .map(|v| {
                        v.iter()
                            .filter(|c| c != &&' ')
                            .collect::<String>()
                            .parse::<u128>()
                            .unwrap_or(0)
                    })
                    .collect::<Vec<u128>>()
                    .iter()
                    .sum::<u128>()
            }
            _ => (),
        }
    }

    Ok(res)
}

fn main() {
    println!("Partie 1: {}", resout_maths().unwrap());
    println!("Partie 2: {}", resout_maths_2().unwrap());
}
