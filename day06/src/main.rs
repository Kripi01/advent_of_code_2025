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

    for i in 1..(n - 1) {
        let v_vec: Vec<i128> = lines_vec[i]
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

fn concat(vec: &[u32]) -> i128 {
    vec.iter().fold(0, |acc, elem| acc * 10 + (*elem as i128))
}

// fn resout_maths_2() -> Result<i128, std::io::Error> {
//     let fichier = std::fs::File::open("input_test")?;
//     let lecteur = std::io::BufReader::new(fichier);
//
//     let numbers_vec: Vec<Vec<String>> = lecteur
//         .lines()
//         .map(|l| {
//             l.unwrap()
//                 .split_whitespace()
//                 .map(|s| s.to_owned())
//                 .collect()
//         })
//         .collect();
//     let n = numbers_vec.len();
//     let op: Vec<String> = numbers_vec[n - 1].clone();
//     let n_op = op.len();
//
//     let mut max_length_col: Vec<usize> = numbers_vec[0].iter().map(|s| s.len()).collect();
//     for i in 1..(n - 1) {
//         for j in 0..n_op {
//             let cur_val = numbers_vec[i][j].clone().len();
//             if max_length_col[j] < cur_val {
//                 max_length_col[j] = cur_val;
//             }
//         }
//     }
//
//     // let mut col_vec: Vec<Vec<Vec<u32>>> = Vec::new();
//     // for _ in 0..n_op {
//     //     col_vec.push(Vec::new());
//     // }
//     // for (c, &max_len) in max_length_col.iter().enumerate() {
//     //     for _ in 0..max_len {
//     //         col_vec[c].push(Vec::new())
//     //     }
//     // }
//
//     // println!("{:?}", col_vec);
//     // for i in 0..(n-1) {
//     //     for j in 0
//     // }
//
//     for i in 0..(n-1) {
//         for j in 0..n_op {
//             let v = numbers_vec[i][j];
//             if j % 2 == 0 {
//                 // Quand le numéro de colonne est pair, on lit
//             } else {
//
//             }
//         }
//     }
//
//     // for i in 0..(n - 1) {
//     //     for (j, v) in lines_vec[i].split_whitespace().enumerate() {
//     //         val_vec.push(Vec::new());
//     //         // let mut k = v.chars().collect::<Vec<char>>().len() - 1;
//     //         let mut k = 0;
//     //         // println!("{}", k);
//     //         v.chars().for_each(|c| {
//     //             if k >= val_vec[j].len() {
//     //                 for _ in 0..=k {
//     //                     // On va jusqu'à k car k vaut taille-1.
//     //                     val_vec[j].push(Vec::new());
//     //                 }
//     //             }
//     //             val_vec[j][k].push(c.to_digit(10).unwrap());
//     //             k += 1;
//     //             // if let Some(new_k) = k.checked_sub(1) {
//     //             //     // k -= 1
//     //             //     k = new_k;
//     //             // }
//     //         })
//     //     }
//     // }
//
//     // println!("{:?}", val_vec);
//
//     let mut res = 0;
//     // for i in 0..n_op {
//     //     match op[i] {
//     //         "*" => res += val_vec[i].iter().map(|v| concat(v)).product::<i128>(),
//     //         "+" => res += val_vec[i].iter().map(|v| concat(v)).sum::<i128>(),
//     //         _ => (),
//     //     }
//     // }
//
//     Ok(res)
// }

fn resout_maths_2() -> Result<i128, std::io::Error> {
    let fichier = std::fs::File::open("input_test")?;
    let lecteur = std::io::BufReader::new(fichier);

    let numbers_vec: Vec<Vec<Vec<char>>> = lecteur
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" ")
                .map(|s| {
                    if s.is_empty() {
                        vec!['_']
                    } else {
                        s.to_owned().chars().collect()
                    }
                })
                .collect()
        })
        .collect();

    println!("{:?}", numbers_vec);
    let n = numbers_vec.len();
    let n_first_col = numbers_vec[0].len();

    let max_length_col: Vec<usize> =
        numbers_vec
            .iter()
            .take(n - 1)
            .fold(Vec::new(), |mut acc, v| {
                acc.push(
                    v.iter()
                        .fold(0, |acc, sub_v| std::cmp::max(sub_v.len(), acc)),
                );
                acc
            });
    println!("{:?}", max_length_col);

    // On applatit un peu numbers_vec.
    let mut flattened_numbers_vec = Vec::new();
    for i in 0..n {
        let flattened_line = numbers_vec[i].iter().map(|v| {
            if v.len() == 3 {
                v
            } else if v.len() < 3 {
                v
            } else {
                panic!("Flattening failed.")
            }
        });
        flattened_numbers_vec.push(flattened_line);
    }

    Ok(0)
}

fn main() {
    println!("{}", resout_maths().unwrap());
    println!("{}", resout_maths_2().unwrap());
}
