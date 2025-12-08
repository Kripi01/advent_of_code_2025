use std::io::BufRead;

fn initialise_descente(diagram: &mut Vec<Vec<char>>, nb_split: &mut u32) {
    let nb_lines = diagram.len();
    let size_line = diagram[0].len();

    for y in 0..nb_lines {
        for x in 0..size_line {
            if diagram[y][x] == 'S' && y + 1 < nb_lines {
                let val_under = diagram[y + 1][x];
                if val_under == '.' {
                    diagram[y + 1][x] = '|';
                } else if val_under == '^' && x >= 1 && x + 1 < size_line {
                    diagram[y + 1][x - 1] = '|';
                    diagram[y + 1][x + 1] = '|';
                    *nb_split += 1;
                }
            }
        }
    }
}

fn propage_rayon(diagram: &mut Vec<Vec<char>>, nb_split: &mut u32) {
    let nb_lines = diagram.len();
    let size_line = diagram[0].len();

    for y in 0..nb_lines {
        for x in 0..size_line {
            if diagram[y][x] == '|' && y + 1 < nb_lines {
                let val_under = diagram[y + 1][x];
                if val_under == '.' {
                    diagram[y + 1][x] = '|';
                } else if val_under == '^' && x >= 1 && x + 1 < size_line {
                    diagram[y + 1][x - 1] = '|';
                    diagram[y + 1][x + 1] = '|';
                    *nb_split += 1;
                }
            }
        }
    }
}

fn trouve_nb_split_tachyon() -> Result<u32, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut diagram = Vec::new();
    for line in lecteur.lines() {
        let line = line?;
        let line_vec: Vec<char> = line.chars().collect();
        diagram.push(line_vec);
    }

    let nb_lines = diagram.len();
    let mut nb_split = 0;
    initialise_descente(&mut diagram, &mut nb_split);

    propage_rayon(&mut diagram, &mut nb_split);

    Ok(nb_split)
}

// -------------------------------------------------------- //
// ----------------------- PARTIE 2 ----------------------- //
// -------------------------------------------------------- //

// fn initialise_descente_timeline(diagram: &Vec<Vec<char>>) -> Option<(usize, usize)> {
//     let nb_lines = diagram.len();
//     let size_line = diagram[0].len();
//
//     for y in 0..nb_lines {
//         for x in 0..size_line {
//             if diagram[y][x] == 'S' && y + 1 < nb_lines {
//                 let val_under = diagram[y + 1][x];
//                 if val_under == '.' {
//                     return Some((y + 1, x));
//                     // pos.push((y + 1, x));
//                     // diagram[y + 1][x] = '|';
//                 }
//             }
//         }
//     }
//
//     None
// }
//
// // On a juste besoin de la dernière position car dans une timeline, il n'y a qu'un seul rayon.
// fn propage_rayon_timeline(diagram: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
//     let nb_lines = diagram.len();
//     let size_line = diagram[0].len();
//
//     // print!("{:?}-", pos);
//     let (mut y, x) = pos;
//     while y + 1 < nb_lines {
//         let val_under = diagram[y + 1][x];
//         if val_under == '.' {
//             y += 1;
//         } else if val_under == '^' && x >= 1 && x + 1 < size_line {
//             let (nb_timeline_left, nb_timeline_right) = rayon::join(
//                 || propage_rayon_timeline(diagram, (y + 1, x - 1)),
//                 || propage_rayon_timeline(diagram, (y + 1, x + 1)),
//             );
//
//             return nb_timeline_left + nb_timeline_right;
//         }
//     }
//
//     1
// }
//
// fn trouve_nb_timelines() -> Result<u32, std::io::Error> {
//     let fichier = std::fs::File::open("input")?;
//     let lecteur = std::io::BufReader::new(fichier);
//
//     let mut diagram = Vec::new();
//     for line in lecteur.lines() {
//         let line = line?;
//         let line_vec: Vec<char> = line.chars().collect();
//         diagram.push(line_vec);
//     }
//
//     let pos = initialise_descente_timeline(&diagram).unwrap();
//
//     Ok(propage_rayon_timeline(&diagram, pos))
// }

fn trouve_nb_timelines() -> Result<u128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut diagram = Vec::new();
    for line in lecteur.lines() {
        let line = line?;
        let line_vec: Vec<char> = line.chars().collect();
        diagram.push(line_vec);
    }

    let nb_lines = diagram.len();
    let size_line = diagram[0].len();

    // On fait de la programmation dynamique en bottom-up.
    let mut nb_timelines = vec![vec![1; size_line]; nb_lines];

    for y in (0..nb_lines).rev() {
        for x in 0..size_line {
            if y + 1 < nb_lines {
                let val_under = diagram[y + 1][x];
                if val_under == '.' {
                    nb_timelines[y][x] = nb_timelines[y + 1][x];
                } else if val_under == '^' && x >= 1 && x + 1 < size_line {
                    let nb_timeline_left = nb_timelines[y + 1][x - 1];
                    let nb_timeline_right = nb_timelines[y + 1][x + 1];
                    nb_timelines[y][x] = nb_timeline_left + nb_timeline_right;
                }
            }
        }
    }

    for y in 0..nb_lines {
        for x in 0..size_line {
            if diagram[y][x] == 'S' {
                return Ok(nb_timelines[y][x]);
            }
        }
    }

    Ok(0)
}

fn main() {
    println!(
        "Le nombre de split est: {}",
        trouve_nb_split_tachyon().unwrap()
    );

    println!(
        "Le nombre de timelines est: {}",
        trouve_nb_timelines().unwrap()
    );
}
