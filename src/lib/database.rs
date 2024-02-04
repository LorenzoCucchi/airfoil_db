use crate::Airfoil;
use bincode;
use polars::prelude::*;
use rusqlite::{params, Connection, Result};
use std::fs;

pub fn create_database(db_name: &str) -> Result<()> {
    let conn = Connection::open(db_name)?;

    let expected_schema = "
        CREATE TABLE IF NOT EXISTS airfoils (
            name TEXT PRIMARY KEY,
            camber REAL,
            camber_pos REAL,
            max_thickness REAL,
            max_thick_pos REAL,
            x_coord TEXT,
            y_coord TEXT
        )";

    // Check if the table already exists
    let table_exists: Result<bool> = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='airfoils'",
        [],
        |row| row.get(0),
    );

    if !table_exists.unwrap_or(false) {
        // Table does not exist, create it
        conn.execute_batch(expected_schema)?;

        return Ok(());
    }

    // Table exists, check if the schema matches
    let existing_schema: Result<String> = conn.query_row(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name='airfoils'",
        [],
        |row| row.get(0),
    );

    if let Ok(existing_schema) = existing_schema {
        if existing_schema.trim() == expected_schema.trim() {
            // Schema matches, return Ok
            return Ok(());
        }
    }

    // Table exists but the schema does not match, return an error
    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete_database(db_name: &str) -> Result<()> {
    if fs::metadata(db_name).is_ok() {
        // Attempt to remove the database file
        if let Err(err) = fs::remove_file(db_name) {
            eprintln!("Error deleting database file: {}", err);
        }
    }

    Ok(())
}

pub fn add_airfoil_to_database(airfoil: &Airfoil, db_name: &str) -> Result<()> {
    let conn = Connection::open(db_name)?;

    conn.execute(
        "INSERT INTO airfoils (name, camber, camber_pos, max_thickness, max_thick_pos, x_coord, y_coord)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &airfoil.name,
            &airfoil.camber,
            &airfoil.camber_pos,
            &airfoil.max_thickness,
            &airfoil.max_thick_pos,
            &airfoil.x_coord.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(","),
            &airfoil.y_coord.iter().map(|&y| y.to_string()).collect::<Vec<String>>().join(",")
        ],
    )?;

    Ok(())
}

pub fn read_airfoil_from_database(db_name: &str) -> Vec<Airfoil> {
    let conn = Connection::open(db_name).unwrap();

    let select_query =
        "SELECT name, camber, camber_pos, max_thickness, max_thick_pos FROM airfoils";

    let mut stmt = conn.prepare(select_query).unwrap();

    let airfoils: Vec<Airfoil> = stmt
        .query_map(params![], |row| {
            let x_coords_str: String = row.get(5)?;
            let y_coords_str: String = row.get(6)?;

            let x_coord: Vec<f64> = x_coords_str
                .split(',')
                .map(|s| s.parse::<f64>().unwrap())
                .collect();
            let y_coord: Vec<f64> = y_coords_str
                .split(',')
                .map(|s| s.parse::<f64>().unwrap())
                .collect();

            Ok(Airfoil {
                name: row.get(0)?,
                camber: row.get(1)?,
                camber_pos: row.get(2)?,
                max_thickness: row.get(3)?,
                max_thick_pos: row.get(4)?,
                x_coord,
                y_coord,
            })
        })
        .and_then(|res| res.collect())
        .unwrap();

    airfoils
}
