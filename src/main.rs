#![allow(dead_code, unused_imports)]
mod camera;
mod geometry;
mod parser;
mod tests;
mod vector;

use crate::{camera::Camera, vector::Vec3};
use regex::Regex;
use std::{
    env::{args, current_dir},
    fs::File,
    io::prelude::*,
    path::Path,
};

fn parse_args() -> (Option<String>, usize, usize) {
    let mut a = args();
    (a.nth(1), 1024, 1024)
}

fn main() {
    let (filename, ..) = parse_args();
    parser::parse(match filename {
        Some(f) => f,
        None => "obj/cyl.obj".to_owned(),
    });
}
