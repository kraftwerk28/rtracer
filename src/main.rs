#![allow(dead_code, unused_imports, unused_variables)]
mod geometry;
mod parser;
mod vector;
mod tracer;

mod vector_test;

use crate::vector::Vec3;
use std::collections::{LinkedList};
// use regex::Regex;
use std::{
    env::{args, current_dir},
    fs::File,
    io::prelude::*,
    path::Path,
};

use parser::*;

fn parse_args() -> (Option<String>, usize, usize) {
    let mut a = args();
    (a.nth(1), 1024, 1024)
}

fn main() {
    let (filename, width, height) = parse_args();
    let parsed = WavefrontObj::parse(match filename {
        Some(f) => f,
        None => "obj/cyl.obj".to_owned(),
    })
    .unwrap();
    print!(
        "Parsing finished. {:?} vertices; {:?} faces.",
        parsed.verts, parsed.faces
    );
}
