use regex::Regex;
use serde::Deserialize;
use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Airfoil {
    pub name: String,
    pub camber: f64,
    pub camber_pos: f64,
    pub max_thickness: f64,
    pub max_thick_pos: f64,
    pub x_coord: Vec<f64>,
    pub y_coord: Vec<f64>,
}

impl Airfoil {
    pub fn from_dat_file(file_path: &str) -> Result<Self, Error> {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let mut coordinates = Vec::new();
        let re = Regex::new(r"\s*([-+]?\d*\.\d+)\s+([-+]?\d*\.\d+)").unwrap();

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(captures) = re.captures(&line) {
                    let x: f64 = captures[1].parse().unwrap();
                    let y: f64 = captures[2].parse().unwrap();
                    coordinates.push((x, y));
                }
            }
        }

        let (camber, max_thickness, camber_x, thick_x) =
            Self::calculate_airfoil_characteristics(&coordinates);

        let name = Path::new(file_path)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Ok(Self {
            name: name, // Use the file name as the default name or modify accordingly
            camber,
            camber_pos: camber_x,
            max_thickness,
            max_thick_pos: thick_x,
            x_coord: coordinates.iter().map(|&(x, _)| x).collect(),
            y_coord: coordinates.iter().map(|&(_, y)| y).collect(),
        })
    }

    fn calculate_airfoil_characteristics(coordinates: &[(f64, f64)]) -> (f64, f64, f64, f64) {
        let mid_index = coordinates.len() / 2;
        let (upper_surface, lower_surface) = coordinates.split_at(mid_index);
        let lower_surface = lower_surface.iter().rev();

        let (camber, camber_x) = upper_surface
            .iter()
            .zip(lower_surface.clone())
            .map(|((x, y_upper), (_, y_lower))| ((y_upper + y_lower) / 2.0, *x))
            .max_by(|&(camber1, _), &(camber2, _)| camber1.partial_cmp(&camber2).unwrap())
            .unwrap_or((0.0, 0.0));

        let (max_thickness, thick_x) = upper_surface
            .iter()
            .zip(lower_surface.clone())
            .map(|((x, y_upper), (_, y_lower))| (y_upper - y_lower, *x))
            .max_by(|&(thickness1, _), &(thickness2, _)| {
                thickness1.partial_cmp(&thickness2).unwrap()
            })
            .unwrap_or((0.0, 0.0));

        (
            camber * 100.0,
            max_thickness.abs() * 100.0,
            camber_x * 100.0,
            thick_x * 100.0,
        )
    }
}
