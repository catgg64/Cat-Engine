use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub mod rect {
    pub struct Rect {
        x: i64,
        y: i64,
        size_x: i64,
        size_y: i64,
    }
    
    impl Rect {
        pub fn new(x: i64, y: i64, size_x: i64, size_y: i64) -> Result<Rect, String>{
            Ok (Self {
                x: x, 
                y: y, 
                size_x: size_x, 
                size_y: size_y,
            })
        }

        pub fn colliderect(&self, rect: Rect) -> bool {
            if self.x > rect.x || self.x < rect.size_x || self.y > rect.y || self.y < rect.size_y{
                true
            }
            else {
                false
            }
        }
        
        pub fn get_x_y_sizex_sizey(&self) -> (i64, i64, i64, i64) {
            (self.x, self.y, self.size_x, self.size_y)
        }
    }
}

pub mod point {
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }
    
    impl Point {
        pub fn new(x: f64, y: f64) -> Result<Point, String> {
            Ok (Self {
                x: x,
                y: y,
            })
        }

        pub fn return_xy(&self) -> (f64, f64) {
            (self.x, self.y)
        }

        pub fn turn_into_sdl_point(&self) -> sdl2::rect::Point {
            sdl2::rect::Point::new(self.x.round() as i32, self.y.round() as i32)
        }
    }
}

#[derive(Debug)]
pub struct ObjData {
    pub vertices: Vec<f32>, // x, y, z, u, v
    pub indices: Vec<u32>,
}

pub fn load_obj<P: AsRef<Path>>(path: P) -> ObjData {
    let file = File::open(path).expect("Failed to open OBJ file");
    let reader = BufReader::new(file);

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut vertex_map = std::collections::HashMap::new();
    let mut vertices: Vec<f32> = Vec::new();
    let mut next_index: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "v" => {
                positions.push([
                    parts[1].parse::<f32>().unwrap(),
                    parts[2].parse::<f32>().unwrap(),
                    parts[3].parse::<f32>().unwrap(),
                ]);
            }
            "vt" => {
                uvs.push([
                    parts[1].parse::<f32>().unwrap(),
                    parts[2].parse::<f32>().unwrap(),
                ]);
            }
            "f" => {
                for &vertex in &parts[1..] {
                    // OBJ face format: vertex/uv/normal
                    let indices_str: Vec<&str> = vertex.split('/').collect();
                    let pos_idx: usize = indices_str[0].parse::<usize>().unwrap() - 1;
                    let uv_idx: usize = indices_str[1].parse::<usize>().unwrap() - 1;

                    // Create a unique vertex key
                    let key = (pos_idx, uv_idx);
                    if let Some(&i) = vertex_map.get(&key) {
                        indices.push(i);
                    } else {
                        let pos = positions[pos_idx];
                        let uv = uvs[uv_idx];
                        vertices.extend_from_slice(&[pos[0], pos[1], pos[2], uv[0], uv[1]]);
                        indices.push(next_index);
                        vertex_map.insert(key, next_index);
                        next_index += 1;
                    }
                }
            }
            _ => {}
        }
    }

    ObjData { vertices, indices }
}