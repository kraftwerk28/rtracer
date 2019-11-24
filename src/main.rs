#![allow(dead_code, unused_imports)]
mod camera;
mod geometry;
mod parser;
mod tests;
mod vector;

use crate::{camera::Camera, vector::Vec3};
use regex::Regex;
use std::{env::current_dir, fs::File, io::prelude::*, path::Path};

fn main() {
    // let re = Regex::new(r"(v|f|vn|vt)\s+(.+)").unwrap();
    // let cap = re.captures("v 34.4 12 -45");
    parser::parse("obj/cyl.obj");
    // let v = Vec3::new(12., -4., 8.);
    // let c = Camera{ pos: Vec3::dummy(), dir: Vec3::dummy(), fov: 60. };
    // println!("{}", v.clone().len());
    // println!("{}", v.clone() + v);
}
