use geo::Contains;
use geo::Polygon;
use geo_types::LineString;
use geo_types::{Rect, coord};
use std::io::BufRead;

#[derive(Clone, Debug)]
struct Point {
    x: u128,
    y: u128,
}

impl Point {
    fn new(x: u128, y: u128) -> Self {
        Point { x, y }
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Debug)]
struct Rectangle {
    top_left: Point,
    bot_right: Point,
}

impl Rectangle {
    fn new(top_left: Point, bot_right: Point) -> Self {
        Rectangle {
            top_left,
            bot_right,
        }
    }

    fn area(self) -> u128 {
        (self.top_left.x.abs_diff(self.bot_right.x) + 1)
            * (self.top_left.y.abs_diff(self.bot_right.y) + 1)
    }
}

// ------------------------------------------ //
// ---------------- PARTIE 1 ---------------- //
// ------------------------------------------ //

fn generate_all_rectangles(
    lecteur: std::io::BufReader<std::fs::File>,
) -> Result<Vec<Rectangle>, std::io::Error> {
    let mut corners_pos = Vec::new();
    for line in lecteur.lines() {
        let line = line?;
        if let Some((left, right)) = line.split_once(',') {
            corners_pos.push(Point::new(
                left.parse::<u128>().unwrap(),
                right.parse::<u128>().unwrap(),
            ));
        }
    }

    let mut all_rectangles = Vec::new();
    for top_left in &corners_pos {
        for bot_right in &corners_pos {
            all_rectangles.push(Rectangle::new(top_left.clone(), bot_right.clone()));
        }
    }

    Ok(all_rectangles)
}

fn find_largest_rectangle() -> Result<u128, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let all_rectangles = generate_all_rectangles(lecteur)?;

    let mut res = 0;
    for rectangle in all_rectangles {
        res = std::cmp::max(res, rectangle.area());
    }

    Ok(res)
}

// ------------------------------------------ //
// ---------------- PARTIE 2 ---------------- //
// ------------------------------------------ //

fn is_valid_rectangle(polygon: &Polygon, rectangle: Rect) -> bool {
    polygon.contains(&rectangle)
}

fn find_largest_rectangle_red_green() -> Result<f64, std::io::Error> {
    let fichier = std::fs::File::open("input")?;
    let lecteur = std::io::BufReader::new(fichier);

    let mut corners_pos = Vec::new();
    for line in lecteur.lines() {
        let line = line?;
        if let Some((left, right)) = line.split_once(',') {
            corners_pos
                .push(coord! {x: left.parse::<f64>().unwrap(), y: right.parse::<f64>().unwrap()});
        }
    }

    let mut all_rectangles = Vec::new();
    for top_left in &corners_pos {
        for bot_right in &corners_pos {
            all_rectangles.push(Rect::new(top_left.clone(), bot_right.clone()));
        }
    }

    let polygon = Polygon::new(LineString::from(corners_pos), Vec::new());
    let all_valid_rectangles = all_rectangles
        .iter()
        .filter(|r| is_valid_rectangle(&polygon, **r))
        .collect::<Vec<&Rect>>();

    let mut res = 0f64;
    for valid_rectangle in all_valid_rectangles {
        res = f64::max(
            res,
            (valid_rectangle.width() + 1f64) * (valid_rectangle.height() + 1f64),
        );
    }

    Ok(res)
}

fn main() {
    println!("Partie 1: {}", find_largest_rectangle().unwrap());
    println!("Partie 2: {}", find_largest_rectangle_red_green().unwrap());
}
