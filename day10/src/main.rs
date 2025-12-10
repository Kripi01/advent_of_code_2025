use good_lp::*;
use ndarray::{Array1, Array2};
use ndarray_linalg::LeastSquaresSvd;
use std::collections::HashSet;
use std::io::BufRead;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

fn get_vects(
    path: String,
) -> Result<(Vec<Vec<usize>>, Vec<Vec<Vec<usize>>>, Vec<Vec<usize>>), std::io::Error> {
    let fichier = std::fs::File::open(path)?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut lights_vec = Vec::new();
    let mut buttons_vec = Vec::new();
    let mut joltages_vec = Vec::new();

    for line in lecteur.lines() {
        let line = line?;

        let line_vec = line
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let n_line = line_vec.len();

        let light_vec = line_vec[0]
            .chars()
            .skip(1)
            .enumerate()
            .filter_map(|(i, c)| match c {
                '#' => Some(i),
                _ => None,
            })
            .collect::<Vec<usize>>();
        lights_vec.push(light_vec);

        let button_vec = line_vec
            .iter()
            .take(n_line - 1)
            .skip(1)
            .map(|s| {
                let n_string = s.chars().count();
                s.chars()
                    .take(n_string - 1)
                    .skip(1)
                    .collect::<String>()
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        buttons_vec.push(button_vec);

        let n_joltage_string = line_vec[n_line - 1].chars().count();
        let joltage_vec = line_vec[n_line - 1]
            .chars()
            .take(n_joltage_string - 1)
            .skip(1)
            .collect::<String>()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        joltages_vec.push(joltage_vec);
    }

    Ok((lights_vec, buttons_vec, joltages_vec))
}

fn int_to_mask(state: usize, max_length: usize) -> Vec<bool> {
    (0..max_length)
        .rev()
        .map(|i| state & (1 << i) != 0)
        .collect()
}

fn filter_vec<T>(v: Vec<T>) -> Vec<T>
where
    T: std::cmp::Eq + std::hash::Hash,
{
    // L'ensemble ne contient que les nombres qui apparaissent un nombre impair de fois dans v.
    let mut set = HashSet::new();
    for x in v {
        if set.contains(&x) {
            set.remove(&x);
        } else {
            set.insert(x);
        }
    }

    set.into_iter().collect()
}

fn find_fewest_button_presses() -> Result<usize, std::io::Error> {
    let (lights_vec, buttons_vec, _) = get_vects("input".to_owned())?;
    let nb_lines = lights_vec.len();

    let mut res = 0;
    // On teste en ne pressant un bouton au maximum qu'une seule fois (c'est suffisant pour cette
    // question).
    for line_idx in 0..nb_lines {
        let mut min_button_presses = usize::MAX;

        let nb_effects = buttons_vec[line_idx].len();

        for i in 0..1 << nb_effects {
            let mask = int_to_mask(i, nb_effects);
            let mut it = mask.iter();
            let mut buttons_line_cloned = buttons_vec[line_idx].clone();
            buttons_line_cloned.retain(|_| *it.next().unwrap());
            let mut cur_vec = buttons_line_cloned
                .into_iter()
                .flatten()
                .collect::<Vec<usize>>();

            // On enlève les occurences paires et on garde les occurences impaires.
            cur_vec = filter_vec(cur_vec);
            // Puis on trie le vecteur pour qu'il soit comparable à lights_vec[line_idx]
            cur_vec.sort();

            if cur_vec == lights_vec[line_idx] {
                let button_presses = mask.iter().fold(0, |acc, b| if *b { acc + 1 } else { acc });
                min_button_presses = std::cmp::min(button_presses, min_button_presses);
            }
        }

        res += min_button_presses;
    }

    Ok(res)
}

// =============================================================== //
// ==== PARTIE 2 MÉGA BRUTE FORCE (NE TERMINE CLAIREMENT PAS) ==== //
// =============================================================== //

// fn solve_line_recursive(
//     buttons: &[Vec<Vec<usize>>],
//     line_idx: usize, // La ligne qu'on traite
//     btn_idx: usize,  // Le bouton qu'on est en train de tester
//     current_voltage: Vec<usize>,
//     target_voltage: &[usize],
//     current_presses: usize, // Nombre d'appuis cumulés
//     min_presses: &mut usize,
// ) {
//     // Si on a déjà fait plus d'appuis que le meilleur score connu, on arrête (Pruning)
//     if current_presses >= *min_presses {
//         return;
//     }
//
//     // Cas de base : on a passé en revue tous les boutons disponibles pour cette ligne
//     if btn_idx == buttons[line_idx].len() {
//         if current_voltage == target_voltage {
//             *min_presses = current_presses;
//         }
//         return;
//     }
//
//     // Récursion : on essaie d'appuyer sur le bouton "btn_idx" 0, 1, 2 fois, etc
//     // On boucle tant qu'on ne dépasse pas le voltage cible.
//     let effects = &buttons[line_idx][btn_idx];
//
//     for k in 0.. {
//         // On calcule le voltage si on appuie k fois
//         let mut next_voltage = current_voltage.clone();
//         let mut possible = true;
//
//         // On applique l'effet k fois (simulé par multiplication ici)
//         // NOTE : Si k=0, la boucle ne fait rien, donc on skippe le bouton.
//         if k > 0 {
//             for _ in 0..k {
//                 for &idx in effects {
//                     if idx < next_voltage.len() {
//                         next_voltage[idx] += 1;
//                     }
//                 }
//             }
//         }
//
//         // On vérifie si on a dépassé la cible.
//         for (v, t) in next_voltage.iter().zip(target_voltage.iter()) {
//             if v > t {
//                 possible = false;
//                 break;
//             }
//         }
//
//         if !possible {
//             // Dans ce cas, on a dépassé la cible, donc on passe au bouton suivant.
//             break;
//         }
//
//         // Si c'est possible, on passe au bouton suivant (btn_idx + 1)
//         solve_line_recursive(
//             buttons,
//             line_idx,
//             btn_idx + 1,
//             next_voltage,
//             target_voltage,
//             current_presses + k,
//             min_presses,
//         );
//     }
// }
//
// fn find_fewest_button_presses_joltage_brute_force(
//     lines: Vec<usize>,
// ) -> Result<usize, std::io::Error> {
//     let (_, buttons_vec, joltages_vec) = get_vects("input".to_owned())?;
//     // let nb_lines = joltages_vec.len();
//
//     // let res = (0..nb_lines)
//     let res = lines
//         .into_par_iter()
//         .map(|line_idx| {
//             let mut min_button_presses = usize::MAX;
//             let target: &Vec<usize> = &joltages_vec[line_idx];
//             let start_voltage = vec![0; target.len()];
//
//             // Lancement de la récursion
//             solve_line_recursive(
//                 &buttons_vec,
//                 line_idx,
//                 0, // On commence par le premier bouton
//                 start_voltage,
//                 target,
//                 0,
//                 &mut min_button_presses,
//             );
//
//             if min_button_presses != usize::MAX {
//                 println!("Ligne {}: {} appuis", line_idx, min_button_presses);
//                 min_button_presses
//             } else {
//                 println!("Ligne {}: Pas de solution trouvée", line_idx);
//                 0
//             }
//         })
//         .sum();
//
//     Ok(res)
// }

// =============================================================== //
// ==== PARTIE 2 AVEC SOLUTION EN RÉEL PUIS BRUTE FORCE POUR ===== //
// ====== PASSER AUX ENTIERS (NE TERMINE PAS, MAIS DE PEU) ======= //
// =============================================================== //

fn check_solution(res_line: &[i64], a: &Array2<f64>, b: &Array1<f64>, error: f64) -> bool {
    if res_line.iter().all(|x| *x >= 0) {
        let diff = a.dot(&Array1::from_iter(res_line.iter().map(|&val| val as f64))) - b;
        if diff.iter().all(|x| x.abs() < error) {
            return true;
        }
    }

    false
}

fn adjust_solution(
    depth: usize,
    res_line: &mut [i64],
    nb_boutons: usize,
    a: &Array2<f64>,
    b: &Array1<f64>,
) -> bool {
    if check_solution(res_line, a, b, 0.001) {
        return true;
    }

    if depth == 0 {
        return false;
    }

    for idx in 0..nb_boutons {
        let perturbations = [-1, 1];
        for &p in &perturbations {
            res_line[idx] += p;
            // println!("{:?}", res_line);

            if res_line[idx] >= 0 && adjust_solution(depth - 1, res_line, nb_boutons, a, b) {
                return true;
            }

            res_line[idx] -= p; // Backtracking
        }
    }

    false
}

fn create_matrix_a(
    nb_indicateurs_joltage: usize,
    buttons_vec: &[Vec<Vec<usize>>],
    line: usize,
) -> Array2<f64> {
    let nb_buttons = buttons_vec[line].len();
    // On remplit la matrice a
    let mut a = Array2::zeros((nb_indicateurs_joltage, nb_buttons));
    for indicateur_idx in 0..nb_indicateurs_joltage {
        // On parcourt tous les boutons à presser.
        for x in 0..nb_buttons {
            if buttons_vec[line][x].contains(&indicateur_idx) {
                a[(indicateur_idx, x)] = 1.0;
            } else {
                a[(indicateur_idx, x)] = 0.0;
            }
        }
    }

    a
}

// En fait, on cherche à résoudre l'équation AX=B avec A une matrice non-nécessairement carrée: il
// y a plus d'inconnues que d'équations.
fn find_fewest_button_presses_joltage() -> Result<u64, std::io::Error> {
    let (_, buttons_vec, joltages_vec) = get_vects("input".to_owned())?;
    let nb_lines = joltages_vec.len();

    let mut res = 0;

    // La matrice A est remplie de la manière suivante:
    // Si le bouton d'indice 0 appartient au premier élément, alors on met un 1 dans la matrice à
    // la colonne 1 (parce que premier élément), ligne 1 (parce qu'indice 0). S'il appartient au
    // deuxième élément, on met un 1 à la colonne 2, ligne 1, etc
    // Si le bouton d'indice 1 appartient au premier élément, alors on met un 1 dans la matrice à
    // la colonne 1 (parce que premier élément), ligne 2 (parce qu'indice 1), ETC
    for line in 0..nb_lines {
        let nb_indicateurs_joltage = joltages_vec[line].len();
        let nb_buttons = buttons_vec[line].len();

        let a = create_matrix_a(nb_indicateurs_joltage, &buttons_vec, line);

        // On remplit le vecteur b.
        let b = Array1::from_iter(joltages_vec[line].iter().map(|&val| val as f64));

        let res_lstsq = a.least_squares(&b).unwrap();
        let sol_vector = res_lstsq.solution.mapv(|x| x.round());

        let mut possible_res_line = sol_vector
            .as_slice()
            .unwrap()
            .iter()
            .map(|x| x.round() as i64)
            .collect::<Vec<i64>>();

        // Il faut ensuite vérifier que notre résultat est le bon, i.e. a * sol_vector = b
        // S'il n'est pas bon, on tente jusqu'à trouver la bonne valeur.
        let search_depth = 6;
        if adjust_solution(search_depth, &mut possible_res_line, nb_buttons, &a, &b) {
            res += 1;
        } else {
            println!("Solution non trouvée pour la ligne: {}", line);
        }
    }

    Ok(res)
}

// =============================================================== //
// ============== PARTIE 2 AVEC ILP SOLVER (TRICHE) ============== //
// =============================================================== //

fn solve_with_ilp(a: &Array2<f64>, b: &Array1<f64>, nb_buttons: usize) -> Option<Vec<i64>> {
    // On crée une liste vide de problèmes.
    let mut problem = ProblemVariables::new();

    // On ajoute les variables (le nombre de fois où l'on presse chaque bouton) au problème.
    // On les force à être des entiers (.integer()), positifs car sa borne inférieure est 0 (.min(0))
    let button_presses: Vec<Variable> = (0..nb_buttons)
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();

    // Objectif : minimiser (voir le solver) le nombre total d'appuis, donc la somme des
    // appuis de chaque boutons.
    // Une Expression représente une expression affine e.g. 2x+3  ou x+y+z
    // objective représente au départ l'expression nulle, puis on y ajoute
    // toutes les variables, donc objective = x_1 + x_2 + ... + x_N
    let mut objective = Expression::from(0);
    for &var in &button_presses {
        objective += var;
    }

    // Une contrainte représente une seule (in)égalité qui doit être vérifiée dans la solution.
    // Ici, les contraintes sont : A * X = B c'est un système à d équations, donc d contraintes.
    let mut constraints = Vec::new();
    for row_idx in 0..b.len() {
        let mut constraint_expr = Expression::from(0);
        for col_idx in 0..nb_buttons {
            let coeff = a[(row_idx, col_idx)];
            constraint_expr += coeff * button_presses[col_idx];
        }
        // On ajoute la contrainte avec eq (pour l'égalité, geq ou leq pour les inégalités).
        constraints.push(constraint_expr.eq(b[row_idx]));
    }

    // On crée un problème de minimisation de l'objectif et on demande à coin_cbc
    let mut solver = problem.minimise(objective).using(coin_cbc);
    for constraint in constraints {
        // On ajoute chacune des contraintes au modèle [le problème ne contient que les variables
        // (et l'objectif)].
        solver = solver.with(constraint);
    }
    let solution = solver.solve();

    match solution {
        Ok(sol) => {
            let result: Vec<i64> = button_presses
                .iter()
                .map(|&var| sol.value(var).round() as i64)
                .collect();
            Some(result)
        }
        Err(_) => None,
    }
}

fn find_fewest_button_presses_joltage_ilp() -> Result<u64, std::io::Error> {
    let (_, buttons_vec, joltages_vec) = get_vects("input".to_owned())?;
    let nb_lines = joltages_vec.len();
    let mut res = 0;
    let mut unsolved_lines = Vec::new();

    for line in 0..nb_lines {
        let nb_indicateurs_joltage = joltages_vec[line].len();
        let nb_buttons = buttons_vec[line].len();

        let a = create_matrix_a(nb_indicateurs_joltage, &buttons_vec, line);
        let b = Array1::from_iter(joltages_vec[line].iter().map(|&val| val as f64));

        match solve_with_ilp(&a, &b, nb_buttons) {
            Some(solution) => {
                let sum: i64 = solution.iter().sum();
                res += sum as u64;
                println!("Ligne {}: {} appuis", line, sum);
            }
            None => {
                println!("Ligne {}: Pas de solution trouvée", line);
                unsolved_lines.push(line);
            }
        }
    }

    if !unsolved_lines.is_empty() {
        println!("Lignes non résolues: {:?}", unsolved_lines);
    }

    Ok(res)
}

fn main() {
    println!(
        "Le nombre minimal d'appuis sur les boutons est: {}",
        find_fewest_button_presses().unwrap()
    );

    println!(
        "Le nombre minimal d'appuis sur les boutons pour le joltage est: {}",
        find_fewest_button_presses_joltage_ilp().unwrap()
    );
}
