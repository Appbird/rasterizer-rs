use crate::util::{Vec4, Vec4Model};


#[derive(Debug, Clone)]
pub struct VertexArrayObject<Attribute> {
    pub attribute:Vec<Attribute>,
    pub idx:Vec<[usize; 3]>
}
#[derive(Debug, Clone)]
pub struct VertColor {
    pub vertex: Vec4Model,
    pub color:Vec4
}  
#[derive(Debug, Clone)]
pub struct TexturedVertex {
    pub vertex: Vec4Model,
    pub uv:Vec4
}  