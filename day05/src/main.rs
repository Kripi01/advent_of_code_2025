use std::cmp;
use std::io::BufRead;
use std::ops::Range;

fn trouve_ingredients_frais() -> Result<u128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut fresh_ranges = Vec::new();
    let mut available_ingredients = Vec::new();
    let mut end_fresh = false;
    for line in lecteur.lines() {
        let line = line?;
        if line.is_empty() {
            end_fresh = true;
            continue;
        }

        if end_fresh {
            available_ingredients.push(line.parse::<i128>().unwrap());
        } else {
            let (begin, end) = line.split_once("-").unwrap();
            fresh_ranges.push((begin.parse::<i128>().unwrap(), end.parse::<i128>().unwrap()));
        }
    }

    let mut res = 0;
    available_ingredients.into_iter().for_each(|i| {
        if fresh_ranges
            .iter()
            .any(|(begin, end)| *begin <= i && i <= *end)
        {
            res += 1;
        }
    });

    Ok(res)
}

fn compte_ingredient_frais() -> Result<u128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut fresh_ranges = Vec::new();
    for line in lecteur.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (begin, end) = line.split_once("-").unwrap();
        fresh_ranges.push(begin.parse::<i128>().unwrap()..(end.parse::<i128>().unwrap() + 1));
    }

    // On trie les plages par ordre croissant de début.
    fresh_ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range<i128>> = Vec::new();
    for r in fresh_ranges {
        if merged.is_empty() {
            merged.push(r);
            continue;
        }

        let last_idx = merged.len() - 1;
        let last = &mut merged[last_idx];

        if r.start <= last.end {
            // On étend last au max des deux fins.
            last.end = cmp::max(last.end, r.end);
        } else {
            // Sinon, c'est une plage disjointe (car fresh_ranges est trié par ordre croissant de
            // début), alors on l'ajoute.
            merged.push(r);
        }
    }

    Ok(merged.iter().fold(0, |mut acc, r| {
        acc += r.end - r.start;
        acc
    }) as u128)
}

fn main() {
    println!("{}", trouve_ingredients_frais().unwrap());
    println!("{}", compte_ingredient_frais().unwrap());
}
