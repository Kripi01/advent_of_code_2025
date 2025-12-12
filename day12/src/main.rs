use core::fmt;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Shape {
    data: Vec<Vec<bool>>,
}

#[derive(Debug, Clone)]
struct RotatedShapes {
    shapes: Vec<Shape>, // Ça doit être un vecteur de 4 éléments.
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    length: usize,
    required_quantities: Vec<usize>,
}

impl Shape {
    fn new() -> Self {
        Shape { data: Vec::new() }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nb_lines = self.data.len();
        let nb_columns = self.data[0].len();
        for y in 0..nb_lines {
            for x in 0..nb_columns {
                write!(f, "{}", if self.data[y][x] { "#" } else { "." })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl RotatedShapes {
    fn new(shape: Shape) -> Self {
        let mut rotated_shapes = Vec::new();
        // let mut rotated_shape = RotatedShapes::new();
        rotated_shapes.push(shape.clone());

        let nb_lines = shape.data.len();
        let nb_columns = shape.data[0].len();

        let mut r_shape1 = Shape::new();
        let mut r_shape2 = Shape::new();
        let mut r_shape3 = Shape::new();
        for y in 0..nb_lines {
            r_shape1.data.push(Vec::new());
            r_shape2.data.push(Vec::new());
            r_shape3.data.push(Vec::new());

            for x in 0..nb_columns {
                r_shape1.data[y].push(shape.data[nb_columns - 1 - x][y]);
                r_shape2.data[y].push(shape.data[nb_lines - 1 - y][nb_columns - x - 1]);
                r_shape3.data[y].push(shape.data[x][nb_lines - 1 - y]);
            }
        }

        rotated_shapes.push(r_shape1);
        rotated_shapes.push(r_shape2);
        rotated_shapes.push(r_shape3);

        RotatedShapes {
            shapes: rotated_shapes,
        }
    }
}

impl fmt::Display for RotatedShapes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for shape in &self.shapes {
            write!(f, "{}", shape)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Region {
    fn new(width: usize, length: usize, required_quantities: Vec<usize>) -> Self {
        Region {
            width,
            length,
            required_quantities,
        }
    }
}

fn get_shapes() -> Result<Vec<Shape>, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut all_shapes = Vec::new();
    for (i, line) in lecteur.lines().enumerate() {
        let line = line?;

        if i % 5 == 0 {
            if line.ends_with(':') {
                all_shapes.push(Shape::new());
                continue;
            } else {
                break;
            }
        }

        if !line.is_empty() {
            let formatted_line = line.chars().map(|c| c == '#').collect::<Vec<bool>>();
            all_shapes[i / 5].data.push(formatted_line);
        }
    }

    Ok(all_shapes)
}

fn get_all_rotated_shapes() -> Result<Vec<RotatedShapes>, std::io::Error> {
    let all_shapes = get_shapes()?;
    let mut all_rotated_shapes = Vec::new();
    for shape in all_shapes {
        all_rotated_shapes.push(RotatedShapes::new(shape));
    }

    Ok(all_rotated_shapes)
}

fn get_all_regions() -> Result<Vec<Region>, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut all_regions = Vec::new();
    let mut start_regions = false;
    for (i, line) in lecteur.lines().enumerate() {
        let line = line?;

        if !start_regions && i % 5 == 0 {
            if line.ends_with(':') {
                continue;
            } else {
                start_regions = true;
            }
        }

        if start_regions {
            let (raw_sizes, quantities) = line.split_once(' ').unwrap();
            let n_sizes_string = raw_sizes.chars().count();
            let sizes = raw_sizes
                .chars()
                .take(n_sizes_string - 1)
                .collect::<String>();
            let (width_str, length_str) = sizes.split_once('x').unwrap();
            let width = width_str.parse::<usize>().unwrap();
            let length = length_str.parse::<usize>().unwrap();

            let required_quantities = quantities
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            all_regions.push(Region::new(width, length, required_quantities));
        }
    }

    Ok(all_regions)
}

// Indique si on peut ajouter la shape à la région et si oui, l'ajoute.
// TODO: passer en vecteurs de coordonnées au lieu de matrices de booléens si ça ne termine pas.
fn add_shape_to_region(
    places_taken: &mut Vec<Vec<bool>>,
    shape: &Shape,
    x_top: usize,
    y_top: usize,
) -> bool {
    let shape_nb_lines = shape.data.len();
    let shape_nb_columns = shape.data[0].len();
    let region_length = places_taken.len();
    let region_width = places_taken[0].len();

    let places_taken_backup = places_taken.clone();
    // On checke les coords.
    if y_top + shape_nb_lines > region_length || x_top + shape_nb_columns > region_width {
        // println!("Out of boundaries");
        return false;
    }

    // Si on arrive ici, c'est que les coords sont bonnes et que la shape fit dans la région.
    for y in 0..shape_nb_lines {
        for x in 0..shape_nb_columns {
            let new_x = x_top + x;
            let new_y = y_top + y;
            let place_taken = places_taken[new_y][new_x];
            let shape_cur = shape.data[y][x];
            if place_taken && shape_cur {
                // Dans ce cas, on ne peut pas placer la shape car il y en a déjà une autre.
                *places_taken = places_taken_backup;
                // println!("Overlap");
                return false;
            } else if !place_taken && shape_cur {
                places_taken[new_y][new_x] = true;
            }
        }
    }

    true
}

fn init_places_taken_from_region(region: &Region) -> Vec<Vec<bool>> {
    let width = region.width;
    let length = region.length;

    let mut places_taken = vec![Vec::new(); length];
    for line in &mut places_taken {
        *line = vec![false; width];
    }

    places_taken
}

fn check_region(region: Region, all_shapes: &Vec<Shape>) -> bool {
    let width = region.width;
    let length = region.length;
    let mut required_quantities = region.required_quantities.clone();

    // Il faut tester pour chaque valeur de l'ajouter à la région.
    let mut places_taken = init_places_taken_from_region(&region);
    for idx in 0..required_quantities.len() {
        let required_quantity = &mut required_quantities[idx];
        let shape = all_shapes[idx].clone();
        let rotated_shapes = RotatedShapes::new(shape);
        while *required_quantity != 0 {
            // println!("idx: {}, required_quantity: {}", idx, required_quantity);
            // for y in 0..places_taken.len() {
            //     for x in 0..places_taken[0].len() {
            //         print!("{}", if places_taken[y][x] { "#" } else { "." });
            //     }
            //     println!();
            // }
            // println!();

            // Pour toutes les positions de la région, on essaie d'ajouter la valeur.
            let mut found = false;
            for x_top in 0..width {
                if found {
                    break;
                }
                for y_top in 0..length {
                    if found {
                        break;
                    }
                    for rotated_shape in &rotated_shapes.shapes {
                        if add_shape_to_region(&mut places_taken, rotated_shape, x_top, y_top) {
                            *required_quantity -= 1;
                            // On passe à la shape suivante, et on revient en arrière.
                            found = true;
                            break;
                        }
                    }
                }
            }

            // Si on l'a trouvé, alors il faut revenir en arrière.
            if found {}
        }
    }

    println!("{:?}", required_quantities);
    required_quantities.into_iter().all(|x| x == 0)
}

fn si_ca_marche_je_pleure(region: Region, all_shapes: &Vec<Shape>) -> bool {
    let region_area = (region.width * region.length) as f64;

    let mut area_of_all_shapes = 0.0;
    for idx in 0..region.required_quantities.len() {
        let quantity_float = region.required_quantities[idx] as f64;
        let shape = &all_shapes[idx];
        let mut shape_area = 0.0;
        for y in 0..shape.data.len() {
            for x in 0..shape.data[0].len() {
                if shape.data[y][x] {
                    shape_area += 1.0;
                }
            }
        }
        area_of_all_shapes += quantity_float * shape_area;
        println!(
            "quantity: {}, shape_area: {}, product: {}, region_area: {}",
            quantity_float,
            shape_area,
            quantity_float * shape_area,
            region_area
        );
    }

    println!(
        "area_of_all_shapes: {}, region_area: {}",
        area_of_all_shapes, region_area
    );

    area_of_all_shapes <= region_area
}

fn main() {
    // println!("{:?}", get_shapes());
    // println!("{:?}", get_all_rotated_shapes());

    // let all_rotated_shapes = get_all_rotated_shapes().unwrap();
    // for rotated_shape in all_rotated_shapes {
    //     println!("{}", rotated_shape);
    // }

    // println!("{:?}", get_all_regions());
    let all_regions = get_all_regions().unwrap();
    let all_rotated_shapes = get_all_rotated_shapes().unwrap();
    let shape1 = &all_rotated_shapes[4].shapes[0];
    let shape2 = &all_rotated_shapes[4].shapes[2];

    let region = &all_regions[0];

    println!("{}", shape1);
    println!("{}", shape2);
    println!("{:?}", region);

    let mut places_taken = init_places_taken_from_region(region);

    let did_fit1 = add_shape_to_region(&mut places_taken, shape1, 0, 0);
    for line in &places_taken {
        println!("{:?}", line);
    }
    println!("{}\n", did_fit1);

    let did_fit2 = add_shape_to_region(&mut places_taken, shape2, 1, 1);
    for line in &places_taken {
        println!("{:?}", line);
    }
    println!("{}\n", did_fit2);

    let all_shapes = get_shapes().unwrap();
    let mut res = 0;
    for region in all_regions {
        // println!("{}", check_region(region, &all_shapes))
        if si_ca_marche_je_pleure(region, &all_shapes) {
            res += 1;
        }
    }
    println!("{}", res);
}
