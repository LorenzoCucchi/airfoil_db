mod lib;
use lib::database::read_airfoil_from_database;

use crate::lib::airfoil::Airfoil;
use crate::lib::database::{add_airfoil_to_database, create_database, delete_database};

fn main() {
    let db_name = "airfoils.db";

    let foil = Airfoil::from_dat_file("SB95 10,5-2.dat").unwrap();
    delete_database(&db_name).unwrap();
    create_database(&db_name).unwrap();
    add_airfoil_to_database(&foil, &db_name).unwrap();
    let foil2 = Airfoil::from_dat_file("ag08.dat").unwrap();
    add_airfoil_to_database(&foil2, &db_name).unwrap();

    let res = read_airfoil_from_database(db_name);

    println!("{:?}", res);
}
