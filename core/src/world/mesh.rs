use crate::util::{Vec4, Vec4Model};


#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec4Model>,
    pub colors: Vec<Vec4>,
    pub v_idx: Vec<[usize; 3]>
}

#[derive(Debug, Clone)]
pub struct TexturedMesh {
    pub vertices: Vec<Vec4Model>,
    pub uv: Vec<Vec4>,
    pub v_idx: Vec<[usize; 3]>
}