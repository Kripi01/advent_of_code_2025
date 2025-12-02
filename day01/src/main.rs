use std::io::BufRead;

fn trouve_nb_zero() -> Result<u32, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut cur_pos = 50;
    lecteur.lines().try_fold(0, |acc, e| {
        let line = e?;

        let mut it = line.split_whitespace();
        let side = it.next().unwrap_or_default();
        let offset: i32 = it.next().unwrap_or_default().parse().unwrap_or_default();

        match side {
            "L" => cur_pos = (cur_pos - offset).rem_euclid(100),
            "R" => cur_pos = (cur_pos + offset).rem_euclid(100),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Pas L ou R.",
                ));
            }
        }

        if cur_pos == 0 { Ok(acc + 1) } else { Ok(acc) }
    })
}

fn trouve_nb_zero_autre_methode() -> Result<i32, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut cur_pos = 50;
    lecteur.lines().try_fold(0, |mut acc, e| {
        let line = e?;

        let mut it = line.split_whitespace();
        let side = it.next().unwrap_or_default();
        let offset: i32 = it.next().unwrap_or_default().parse().unwrap_or_default();

        match side {
            "L" => {
                let new_pos = cur_pos - offset;
                if new_pos <= 0 && cur_pos != 0 {
                    acc += ((new_pos / 100).abs()) + 1;
                }
                // Si on est à 0, alors un -100 ne fait un offset que de 1 (le 0 initial a été
                // compté à l'itération précédente).
                if new_pos <= 0 && cur_pos == 0 {
                    acc += (new_pos / 100).abs();
                }

                cur_pos = new_pos.rem_euclid(100);
            }
            "R" => {
                let new_pos = cur_pos + offset;
                if new_pos >= 100 {
                    acc += new_pos / 100;
                }
                cur_pos = new_pos.rem_euclid(100);
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Pas L ou R.",
                ));
            }
        }

        Ok(acc)
    })
}

fn main() {
    println!(
        "Le nombre de fois où le dial passe par zero est: {}",
        trouve_nb_zero().unwrap_or_default()
    );

    println!(
        "Le mot de passe avec la méthode 0x434C49434B est: {}",
        trouve_nb_zero_autre_methode().unwrap_or_default()
    );
}
