use crate::{geometry::Face, vector::Vec3 as Vec3_};
use std::{
    env::current_dir,
    fmt,
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
    rc::Rc,
};

type Vec3 = Vec3_<f64>;

pub struct WavefrontObj {
    pub verts: Vec<Rc<Vec3>>,
    pub faces: Vec<Face>,
}

pub struct ParseError(String);

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing .obj. {}", self.0)
    }
}

impl WavefrontObj {
    pub fn init() -> Self {
        Self {
            verts: Vec::new(),
            faces: Vec::new(),
        }
    }
    pub fn add_vert(&mut self, v: Vec3) -> &Self {
        self.verts.push(Rc::new(v));
        self
    }
    pub fn add_face(&mut self, f: Face) -> &Self {
        self.faces.push(f);
        self
    }
    pub fn verts_count(&self) -> usize {
        self.verts.len()
    }
    pub fn faces_count(&self) -> usize {
        self.faces.len()
    }
    pub fn parse(filename: String) -> Result<Self, ParseError> {
        let f_path =
            Path::new(current_dir().unwrap().as_os_str()).join(filename);
        let f = std::fs::File::open(f_path).unwrap();
        let br = BufReader::new(f);
        let mut result = Self::init();
        let mut v_indexes: Vec<[usize; 3]> = Vec::new();
        for line in br.lines() {
            let l = line.unwrap();
            let mut tokens = l.split_whitespace().map(|s| s.trim());
            match tokens.nth(0).unwrap() {
                "v" => {
                    let coords: Vec<f64> =
                        tokens.take(3).map(|s| s.parse().unwrap()).collect();
                    result.add_vert(Vec3::from_vec(coords));
                }
                "f" => {
                    let face: Vec<usize> = tokens
                        .take(3)
                        .map(|s| {
                            let f64_chars: String = s
                                .chars()
                                .take_while(|c| c.is_numeric())
                                .collect();
                            f64_chars.parse().unwrap()
                        })
                        .collect();
                    if face.len() < 3 {
                        return Err(ParseError(
                            "Not enough face vertices".to_owned(),
                        ));
                    }
                    v_indexes.push([face[0], face[1], face[2]]);
                }
                _ => {}
            };
        }

        for v_i in v_indexes {
            let v1 = result.verts[v_i[0] - 1].clone();
            let v2 = result.verts[v_i[1] - 1].clone();
            let v3 = result.verts[v_i[2] - 1].clone();
            let face = Face::new(v1, v2, v3);
            result.add_face(face);
        }

        Ok(result)
    }
}
