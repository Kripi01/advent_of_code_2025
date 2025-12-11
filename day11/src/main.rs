use std::{collections::HashMap, io::BufRead};

fn get_problem_vects(
    path: String,
    real_start_device: String,
) -> Result<(Vec<String>, Vec<Vec<String>>, usize), std::io::Error> {
    let fichier = std::fs::File::open(path)?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut start_devices = Vec::new();
    let mut attached_devices = Vec::new();
    let mut start_idx = 0;

    for (i, line) in lecteur.lines().enumerate() {
        let line = line?;

        let (start_device, attached_device) = line.split_once(':').unwrap();
        if start_device == real_start_device {
            start_idx = i;
        }
        start_devices.push(start_device.to_owned());

        attached_devices.push(
            attached_device
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        );
    }

    Ok((start_devices, attached_devices, start_idx))
}

fn solve_recursive(
    cur_device_idx: usize,
    start_devices: &[String],
    attached_devices: &[Vec<String>],
    device_output: &String,
) -> u128 {
    let mut nb_paths = 0;
    for attached_device in &attached_devices[cur_device_idx] {
        // Il nous faut l'indice de la machine.
        // TODO: Utiliser une hashMap pour ne pas s'embêter avec les indices.
        if attached_device == device_output {
            nb_paths += 1;
        } else if let Some(new_start_idx) = start_devices.iter().position(|n| n == attached_device)
        {
            nb_paths += solve_recursive(
                new_start_idx,
                start_devices,
                attached_devices,
                device_output,
            );
        } else {
            panic!("Format de données invalides: sortie (autre que out) qui n'est pas une entrée.");
        }
    }

    nb_paths
}

fn get_nb_path() -> Result<u128, std::io::Error> {
    let real_start_device = "you".to_owned();
    let device_output = "out".to_owned();
    let (start_devices, attached_devices, start_idx) =
        get_problem_vects("input".to_owned(), real_start_device)?;

    let res = solve_recursive(start_idx, &start_devices, &attached_devices, &device_output);

    Ok(res)
}

// ====================================================== //
// ====================== PARTIE 2 ====================== //
// ====================================================== //

// La mémoisation se base sur le fait que 2 chemins qui ont le même
// nombre de contraintes et qui arrivent au même endroit peuvent être
// considérés comme identiques.
fn solve_recursive_constrained_memoization<'a>(
    cur_device: &'a String,
    devices_hm: &'a HashMap<&String, &Vec<String>>,
    device_output: &String,
    constraints_encountered: usize,
    memo: &mut HashMap<(&'a String, usize), u128>,
) -> u128 {
    let mut new_constraints_encountered = constraints_encountered;
    if cur_device == "dac" || cur_device == "fft" {
        new_constraints_encountered += 1;
    }

    if new_constraints_encountered > 2 {
        return 0;
    }

    if cur_device == device_output {
        if new_constraints_encountered == 2 {
            return 1;
        } else {
            return 0;
        }
    }

    // On checke si on connaît déjà le résultat.
    if let Some(&nb_paths) = memo.get(&(cur_device, new_constraints_encountered)) {
        return nb_paths;
    }

    // Sinon, on fait le calcul récursif puis on ajoute à la HashMap.
    let mut nb_paths = 0;
    if let Some(neighbors) = devices_hm.get(cur_device) {
        for attached_device in *neighbors {
            nb_paths += solve_recursive_constrained_memoization(
                attached_device,
                devices_hm,
                device_output,
                new_constraints_encountered,
                memo,
            );
        }
    }
    memo.insert((cur_device, new_constraints_encountered), nb_paths);

    nb_paths
}

fn get_nb_path_constrained() -> Result<u128, std::io::Error> {
    let real_start_device = "svr".to_owned();
    let device_output = "out".to_owned();
    let (start_devices, attached_devices, _) =
        get_problem_vects("input".to_owned(), real_start_device.clone())?;

    // On crée ensuite la hashMap.
    let mut devices_hm = HashMap::new();
    for start_device_idx in 0..start_devices.len() {
        devices_hm.insert(
            &start_devices[start_device_idx],
            &attached_devices[start_device_idx],
        );
    }

    let mut memo = HashMap::new();
    let res = solve_recursive_constrained_memoization(
        &real_start_device,
        &devices_hm,
        &device_output,
        0,
        &mut memo,
    );

    Ok(res)
}

fn main() {
    println!(
        "Le nombre de chemins de you à out est: {}",
        get_nb_path().unwrap()
    );

    println!(
        "Le nombre de chemins de svr à out passant par dac ET fft est: {}",
        get_nb_path_constrained().unwrap()
    );
}
