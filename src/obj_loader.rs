use nalgebra_glm::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ObjModel {
    pub vertices: Vec<Vec3>,
}

impl ObjModel {
    pub fn load(path: &str) -> Self {
        let file = File::open(path).expect("No se pudo abrir el .obj");
        let reader = BufReader::new(file);
        let mut vertices = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("v ") {
                let parts: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect();
                vertices.push(Vec3::new(parts[0], parts[1], parts[2]));
            }
        }

        Self { vertices }
    }
}
