use std::{fs::File, i32, io::{BufRead, BufReader}};

use std::collections::HashMap;

#[derive(Debug)]
#[repr(C)]
pub struct MeshVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
}

impl MeshVertex {
    pub fn new(x: f32, y: f32, z: f32, uv: (f32, f32)) -> Self {
        MeshVertex { x, y, z, u: uv.0, v: uv.1 }
    }
}
pub struct Mesh {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub indicies: Vec<u32>,
    pub texture: super::video::surface::Surface,
    animations: HashMap<String, ProssesedAnimation>,
    current_animation: Option<String>,
    obj_data: ObjData,
}

impl Mesh {
    pub fn new(objdata: ObjData, surface: super::video::surface::Surface) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (objdata.vertices.len() * std::mem::size_of::<MeshVertex>()) as isize,
                objdata.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (objdata.indices.len() * std::mem::size_of::<u32>()) as isize,
                objdata.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = (5 * std::mem::size_of::<f32>()) as i32;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);
        }

        Self { vao, vbo, ebo, indicies: objdata.indices.clone(), texture: surface, animations: HashMap::new(), current_animation: None, obj_data: objdata }
    }

    pub fn new_from_texture_file(objdata: ObjData, texture: &str) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (objdata.vertices.len() * std::mem::size_of::<f32>()) as isize,
                objdata.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (objdata.indices.len() * std::mem::size_of::<u32>()) as isize,
                objdata.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = (5 * std::mem::size_of::<f32>()) as i32;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);
        }
        let used_texture = super::video::surface::Surface::from_texture(&texture);
        
        Self { vao, vbo, ebo, indicies: objdata.indices.clone(), texture: used_texture, animations: HashMap::new(), current_animation: None, obj_data: objdata }
    }

    pub fn append_animation(&mut self, animation: Animation, name: String) {
        self.animations.insert(name, ProssesedAnimation::new(animation.animation_objects));
    }

    pub fn play_animation(&mut self, animation_name: String) {
        self.current_animation = Some(animation_name)      
    }

    pub fn update_animations(&mut self, tick: f64) {
        match self.current_animation {
            None => {},
            Some(_) => {
                let used_animation = self.animations.get(&self.current_animation.clone().unwrap()).unwrap();

                for indice in &self.obj_data.indices {
                    match used_animation.animation_objects.get(indice) {
                        None => {},
                        Some(value) => {
                            let mut before: Option<(String, u64)> = None;
                            let mut after: Option<(String, u64)> = None;

                            for v in value {
                                let time = v.1;

                                // Nothing AI suspisious in here.....
                                if time <= tick as u64 {
                                    if before.is_none() || time > before.as_ref().unwrap().1 {
                                        before = Some((v.0.clone(), time));
                                    }
                                } else {
                                    if after.is_none() || time < after.as_ref().unwrap().1 {
                                        after = Some((v.0.clone(), time));
                                    }
                                }
                                
                                match before {
                                    None => {panic!("There must be a value of keyframe type 1!")}
                                    Some(ref before) => {
                                        match after {
                                            None => {}
                                            Some(ref after) => {
                                                let after_split: Vec<&str> = after.0.split(" ").collect();
                                                let (after_x, after_y, after_z, after_u, after_v): (f32, f32, f32, f32, f32) = (after_split[0].to_string().parse().unwrap(), after_split[1].to_string().parse().unwrap(), after_split[2].to_string().parse().unwrap(), after_split[3].to_string().parse().unwrap(), after_split[4].to_string().parse().unwrap());
                                                let before_split: Vec<&str> = before.0.split(" ").collect();
                                                let (before_x, before_y, before_z, before_u, before_v): (f32, f32, f32, f32, f32) = (before_split[0].to_string().parse().unwrap(), before_split[1].to_string().parse().unwrap(), before_split[2].to_string().parse().unwrap(), before_split[3].to_string().parse().unwrap(), before_split[4].to_string().parse().unwrap());
                                                let duration = (after.1 - before.1) as f64;
                                                let relative_current_time = (tick - before.1 as f64) / duration;
                                                let x = before_x as f64 + (after_x - before_x) as f64 * relative_current_time;
                                                let y = before_y as f64 + (after_y - before_y) as f64 * relative_current_time;
                                                let z = before_z as f64 + (after_z - before_z) as f64 * relative_current_time;
                                                let u = before_u as f64 + (after_u - before_u) as f64 * relative_current_time;
                                                let v = before_v as f64 + (after_v - before_v) as f64 * relative_current_time;

                                                self.obj_data.vertices[*indice as usize].x = x as f32;
                                                self.obj_data.vertices[*indice as usize].y = y as f32;
                                                self.obj_data.vertices[*indice as usize].z = z as f32;
                                                self.obj_data.vertices[*indice as usize].u = u as f32;
                                                self.obj_data.vertices[*indice as usize].v = v as f32;

                                            }
                                        }
                                    }
                                }
                                
                                
                            };
                            
                            //println!("Hi! I'm on {:#?}!", value);
                            
                        }
                    }
                }
                unsafe {
                    gl::BindVertexArray(self.vao);

                    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (self.obj_data.vertices.len() * std::mem::size_of::<MeshVertex>()) as isize,
                        self.obj_data.vertices.as_ptr() as *const _,
                        gl::STATIC_DRAW,
                    );

                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                    gl::BufferData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        (self.obj_data.indices.len() * std::mem::size_of::<u32>()) as isize,
                        self.obj_data.indices.as_ptr() as *const _,
                        gl::STATIC_DRAW,
                    );

                    let stride = (5 * std::mem::size_of::<f32>()) as i32;
                    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
                    gl::EnableVertexAttribArray(1);
                }
            },
        }
    }

    //pub fn update(&mut self) {
    //    update_uv_3d_element_array(&mut self.vao, &mut self.vbo, &mut self.ebo, &mut self., vertices, uvs, indicies);
    //}
}

#[derive(Debug)]
pub struct ObjData {
    pub vertices: Vec<MeshVertex>,
    pub indices: Vec<u32>,
}

impl ObjData {
    pub fn new(vertices: Vec<MeshVertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn from_obj(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open OBJ file");
        let reader = BufReader::new(file);

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertex_map: HashMap<(usize, usize), u32> = std::collections::HashMap::new();
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
                    let face_vertices: Vec<(usize, usize)> = parts[1..].iter().map(|&vertex| {
                        let indices_str: Vec<&str> = vertex.split('/').collect();
                        let pos_idx = indices_str[0].parse::<usize>().unwrap() - 1;
                        let uv_idx = indices_str[1].parse::<usize>().unwrap() - 1;
                        (pos_idx, uv_idx)
                    }).collect();

                    for i in 1..(face_vertices.len() - 1) {
                        for &idx in &[0, i, i + 1] {
                            let (pos_idx, uv_idx) = face_vertices[idx];
                            let pos = positions[pos_idx];
                            let uv = uvs[uv_idx];
                            vertices.extend_from_slice(&[pos[0], pos[1], pos[2], uv[0], 1.0 - uv[1]]);
                            indices.push(next_index);
                            next_index += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        let mut used_verticies = Vec::new();
        for vertex in vertices.chunks(5){
            used_verticies.push(MeshVertex::new(vertex[0], vertex[1], vertex[2], (vertex[3], vertex[4])));
        }

        println!("first 3 vt lines in OBJ should be:");
        println!("  uv 0: (0.9999998, 0.0647047)");
        println!("  uv 1: (0.9826624, 0.0)");
        println!("  uv 2: (0.8705904, 0.0647047)");
        println!("parsed uvs:");
        for i in 0..3 {
            println!("  uv {}: ({}, {})", i, vertices[i*5+3], vertices[i*5+4]);
        }

        ObjData { vertices: used_verticies, indices }
    }
}

pub struct Animation {
    animation_objects: Vec<(u32, MeshVertex, u64)>, // Indice -> Vertex + Time
}

impl Animation {
    pub fn new(animation_objects: Vec<(u32, MeshVertex, u64)>) -> Self {
        Self { animation_objects }
    }
}

#[derive(std::fmt::Debug)]
pub struct ProssesedAnimation {
    animation_objects: HashMap<u32, Vec<(String, u64)>>,
}

impl ProssesedAnimation {
    pub fn new(animation_objects: Vec<(u32, MeshVertex, u64)>) -> Self {
        // This is quite complicated so let me explain it here:
        // The processed_animation is a HashMap. This HashMap contais the value for each individual indices, being the "threads".
        // If an has no HashMap attached to it, it creates a new one and stores itself. If it has, then it just adds it's value to it.

        let mut processed_animation: HashMap<u32, Vec<(String, u64)>> = HashMap::new(); // The String value is just for identifing the Vertex, it doesn't actually have any effect.

        // HashMap (the threads) that contains a hash map (thread) that contains the new mesh value and it's time.

        for keyframe in animation_objects.iter() {
            println!("processing value {:#?}...", keyframe); 
            match processed_animation.get_mut(&keyframe.0) {
                None => {
                    println!("no value indexed as {:#?}, creating a new one.", keyframe.0);
                    processed_animation.insert(keyframe.0.to_owned(), vec![]);
                    processed_animation.get_mut(&keyframe.0).unwrap().push((
                        format!("{} {} {} {} {}", 
                        keyframe.1.x, keyframe.1.y, keyframe.1.z, keyframe.1.u, keyframe.1.v), 
                        keyframe.2.to_owned())); // They need to be Strings because of sum rust shi
                }
                Some(v) => {
                    println!("pushing value: {:#?}, since there is already {}", v, keyframe.0);
                    v.push(
                        (format!("{} {} {} {} {}", 
                        keyframe.1.x, keyframe.1.y, keyframe.1.z, keyframe.1.u, keyframe.1.v), 
                        keyframe.2));
                    //processed_animation.get_mut(&keyframe.0).unwrap().push((
                    //    format!("{} {} {} {} {}", 
                    //    keyframe.1.x, keyframe.1.y, keyframe.1.z, keyframe.1.u, keyframe.1.v), 
                    //    keyframe.0.to_owned())); // They need to be Strings because of sum rust shi
                }
            }
            
        }

        println!("finar result: {:#?}", processed_animation);

        Self { animation_objects: processed_animation }
    }
}