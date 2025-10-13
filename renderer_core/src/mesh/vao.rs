#[derive(Debug, Clone)]
pub struct VertexArrayObject<Attribute> {
    pub attribute:Vec<Attribute>,
    pub idx:Vec<[usize; 3]>
}