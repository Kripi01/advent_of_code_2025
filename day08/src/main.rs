use std::cmp::Ordering;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

fn initialize_circuits() -> Result<Vec<Vec<Point>>, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut circuits = Vec::new();
    for line in lecteur.lines() {
        let line = line?;
        let v_point = line
            .split(",")
            .map(|coord| coord.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        if v_point.len() != 3 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Mauvaises coordonnées de points.",
            ));
        }

        circuits.push(vec![Point {
            x: v_point[0],
            y: v_point[1],
            z: v_point[2],
        }])
    }

    Ok(circuits)
}

fn euclidian_distance(p1: &Point, p2: &Point) -> f64 {
    let diff_x = (p1.x - p2.x) as f64;
    let diff_y = (p1.y - p2.y) as f64;
    let diff_z = (p1.z - p2.z) as f64;
    (diff_x * diff_x + diff_y * diff_y + diff_z * diff_z).sqrt()
}

fn find_circuit_index(circuits: &Vec<Vec<Point>>, pt: &Point) -> Option<usize> {
    for i in 0..circuits.len() {
        if circuits[i].contains(pt) {
            return Some(i);
        }
    }
    None
}

fn get_shortest_connections(n: usize) -> Result<usize, std::io::Error> {
    let mut circuits = initialize_circuits().unwrap();
    let all_points = circuits
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<Point>>();

    let mut all_pairs = Vec::new();
    for i in 0..all_points.len() {
        for j in (i + 1)..all_points.len() {
            let p1 = all_points[i].clone();
            let p2 = all_points[j].clone();
            let dist = euclidian_distance(&p1, &p2);
            all_pairs.push((dist, p1, p2))
        }
    }

    all_pairs.sort_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap_or(Ordering::Equal));

    for k in 0..n {
        let (_, p1, p2) = &all_pairs[k];

        // On cherche où sont ces points dans les circuits actuels (car ils ont pu être déplacés
        // aux itérations précédentes).
        let idx1 = find_circuit_index(&circuits, p1);
        let idx2 = find_circuit_index(&circuits, p2);

        if let (Some(i1), Some(i2)) = (idx1, idx2) {
            // Si les boxes sont dans des circuits différents alors on fusionne.
            if i1 != i2 {
                let mut new_circuits = circuits[i1].clone();
                new_circuits.append(&mut circuits[i2]);
                circuits[i1] = new_circuits;
            }
        }
    }

    let max_size_vec = circuits.iter().fold(vec![0, 0, 0], |mut acc, v| {
        let v_len = v.len();
        let lowest_max_index = acc
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();
        if v_len > acc[lowest_max_index] {
            acc[lowest_max_index] = v_len;
        }
        acc
    });

    Ok(max_size_vec.iter().product())
}

fn get_distance_wall() -> Result<isize, std::io::Error> {
    let mut circuits = initialize_circuits().unwrap();
    let all_points = circuits
        .clone()
        .into_iter()
        .flatten()
        .collect::<Vec<Point>>();

    let mut all_pairs = Vec::new();
    for i in 0..all_points.len() {
        for j in (i + 1)..all_points.len() {
            let p1 = all_points[i].clone();
            let p2 = all_points[j].clone();
            let dist = euclidian_distance(&p1, &p2);
            all_pairs.push((dist, p1, p2))
        }
    }

    all_pairs.sort_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap_or(Ordering::Equal));

    let mut k = 0;
    let n_max_circuit = circuits.len();
    let mut last_couple = None;
    while !circuits.iter().any(|v| v.len() == n_max_circuit) {
        let (_, p1, p2) = &all_pairs[k];

        // On cherche où sont ces points dans les circuits actuels (car ils ont pu être déplacés
        // aux itérations précédentes).
        let idx1 = find_circuit_index(&circuits, p1);
        let idx2 = find_circuit_index(&circuits, p2);
        last_couple = Some((p1, p2));

        if let (Some(i1), Some(i2)) = (idx1, idx2) {
            // Si les boxes sont dans des circuits différents alors on fusionne.
            if i1 != i2 {
                let mut new_circuits = circuits[i1].clone();
                new_circuits.append(&mut circuits[i2]);
                circuits[i1] = new_circuits;
            }
        }
        k += 1;
    }

    if let Some((p1, p2)) = last_couple {
        Ok(p1.x * p2.x)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "ouille",
        ))
    }
}

fn main() {
    println!(
        "Le produit des tailles des 3 plus grandes connexions est: {}",
        get_shortest_connections(1000).unwrap()
    );

    println!("La distance au mur est: {}", get_distance_wall().unwrap());
}
