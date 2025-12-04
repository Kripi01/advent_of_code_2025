use std::io::BufRead;

fn count_adjacent_rolls(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut res = 0;
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in directions {
        let neigh_x = x as isize + dx;
        let neigh_y = y as isize + dy;
        if neigh_x >= 0
            && neigh_x < width
            && neigh_y >= 0
            && neigh_y < height
            && grid[neigh_y as usize][neigh_x as usize] == '@'
        {
            res += 1;
        }
    }

    res
}

fn count_forklifts() -> Result<u32, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let grid: Vec<Vec<char>> = lecteur
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut res = 0;

    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' && count_adjacent_rolls(&grid, x, y) < 4 {
                res += 1;
            }
        }
    }

    Ok(res)
}

fn remove_forklifts(grid: &mut Vec<Vec<char>>) -> u32 {
    let mut nb_removed_rolls = 0;
    let mut removed_rolls_pos = Vec::new();

    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' && count_adjacent_rolls(&grid, x, y) < 4 {
                nb_removed_rolls += 1;
                removed_rolls_pos.push((x, y));
            }
        }
    }

    for (x, y) in removed_rolls_pos {
        grid[y][x] = '.';
    }

    nb_removed_rolls
}

fn loop_forklifts() -> Result<u32, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut grid: Vec<Vec<char>> = lecteur
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut res_tot = 0;
    loop {
        let nb_removed_rolls = remove_forklifts(&mut grid);
        res_tot += nb_removed_rolls;
        if nb_removed_rolls == 0 {
            break;
        }
    }

    Ok(res_tot)
}

fn main() {
    println!("Question 1: {}", count_forklifts().unwrap());
    println!("Question 2: {}", loop_forklifts().unwrap());
}
