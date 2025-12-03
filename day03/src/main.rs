use std::io::BufRead;

fn trouve_voltage() -> Result<u32, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut res = 0;
    for line in lecteur.lines() {
        let line = line?;

        let mut ma = 0;
        let line_vec: Vec<char> = line.chars().collect();
        for (i, c1) in line_vec.iter().enumerate() {
            let cur_int_i = c1.to_digit(10).unwrap();
            for c2 in line.chars().skip(i + 1) {
                let cur_int_j = c2.to_digit(10).unwrap();
                let cur = 10 * cur_int_i + cur_int_j;
                if cur > ma {
                    ma = cur;
                }
            }
        }

        res += ma;
    }

    Ok(res)
}

fn trouve_voltage_12() -> Result<u128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut res = 0;
    for line in lecteur.lines() {
        let line = line?;

        let mut ma_vec: Vec<char> = line.chars().take(12).collect();

        for c in line.chars().skip(12) {
            let ma_s: String = ma_vec.iter().collect();
            let ma_int = ma_s.parse::<u128>().unwrap();

            // On enlève un élément du vecteur en commençant par le début
            // (car au début les chiffres ont plus de poids).
            for k in 0..12 {
                let mut new_ma_vec = ma_vec.clone();
                new_ma_vec.remove(k).to_digit(10).unwrap();

                new_ma_vec.push(c);

                let new_ma_s: String = new_ma_vec.iter().collect();
                let new_ma_int = new_ma_s.parse::<u128>().unwrap();

                if new_ma_int > ma_int {
                    ma_vec = new_ma_vec;
                    break;
                }
            }
        }

        res += ma_vec.iter().collect::<String>().parse::<u128>().unwrap();
    }

    Ok(res)
}

fn main() {
    let res1 = trouve_voltage().unwrap();
    println!("{res1}");

    let res2 = trouve_voltage_12().unwrap();
    println!("{res2}");
}
