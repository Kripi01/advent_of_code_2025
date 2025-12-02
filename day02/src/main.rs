use std::io::BufRead;
use std::iter::zip;

fn is_invalid_id(id: i128) -> bool {
    let s_id = id.to_string();
    let n = s_id.chars().count();
    if n % 2 == 1 {
        return false;
    }

    let it_begin = s_id.chars().take(n / 2);
    let it_end = s_id.chars().skip(n / 2);
    zip(it_begin, it_end).all(|(c1, c2)| c1 == c2)
}

fn trouve_somme_id() -> Result<i128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut somme = 0;
    for line in lecteur.lines() {
        let line = line?;
        for range in line.split(",") {
            let mut it = range.split("-");
            let (begin, end) = (it.next().unwrap_or_default(), it.next().unwrap_or_default());

            let begin_nb: i128 = begin.parse().unwrap_or_default();
            let end_nb: i128 = end.parse().unwrap_or_default();

            for i in begin_nb..=end_nb {
                if is_invalid_id(i) {
                    somme += i;
                }
            }
        }
    }

    Ok(somme)
}

fn is_invalid_id_part2(id: i128) -> bool {
    let s_id = id.to_string();
    let s_vec: Vec<char> = s_id.chars().collect();

    let n = s_id.chars().count();

    for m in 1..n {
        // Pour chaque diviseur de n
        if n % m == 0 {
            if m == 1 {
                // Il faut que tous les éléments du s_vec soient égaux.
                if s_vec.windows(2).all(|w| w[0] == w[1]) {
                    return true;
                }
            } else {
                // On fait comme si le string était coupé en m parties de tailles égales.
                let chunk_size = n / m;
                let mut slice_vec = Vec::new();
                for i in 0..m {
                    let mut pattern = Vec::new();
                    for j in 0..chunk_size {
                        pattern.push(s_vec[i * chunk_size + j]);
                    }
                    slice_vec.push(pattern);
                }

                if slice_vec.windows(2).all(|w| w[0] == w[1]) {
                    // Si tous les éléments du slice_vec sont égaux.
                    // On a trouvé une découpe qui convenait.
                    return true;
                }
            }
        }
    }

    false
}

fn trouve_somme_id_part2() -> Result<i128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut somme = 0;
    for line in lecteur.lines() {
        let line = line?;
        for range in line.split(",") {
            let mut it = range.split("-");
            let (begin, end) = (it.next().unwrap_or_default(), it.next().unwrap_or_default());

            let begin_nb: i128 = begin.parse().unwrap_or_default();
            let end_nb: i128 = end.parse().unwrap_or_default();

            for i in begin_nb..=end_nb {
                if is_invalid_id_part2(i) {
                    somme += i;
                }
            }
        }
    }

    Ok(somme)
}

fn main() {
    // Partie 1
    assert!(is_invalid_id(11));
    assert!(is_invalid_id(22));
    assert!(is_invalid_id(99));
    assert!(is_invalid_id(1188511885));
    assert!(is_invalid_id(446446));
    assert!(is_invalid_id(38593859));
    assert!(!is_invalid_id(38593856));
    assert!(!is_invalid_id(1227775554));
    assert!(!is_invalid_id(0101));

    let somme = trouve_somme_id().unwrap_or_default();
    println!("La somme des IDs invalides est: {somme}");

    // Partie 2
    assert!(is_invalid_id_part2(11));
    assert!(is_invalid_id_part2(111));
    assert!(is_invalid_id_part2(1010));
    assert!(is_invalid_id_part2(999));
    assert!(is_invalid_id_part2(1188511885));
    assert!(is_invalid_id_part2(446446));
    assert!(is_invalid_id_part2(38593859));
    assert!(is_invalid_id_part2(2121212121));
    assert!(!is_invalid_id_part2(38593856));
    assert!(!is_invalid_id_part2(1227775554));
    assert!(!is_invalid_id_part2(0101));

    let somme2 = trouve_somme_id_part2().unwrap_or_default();
    println!("La somme des IDs invalides est: {somme2}");
}
