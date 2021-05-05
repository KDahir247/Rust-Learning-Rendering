//reminder: need to include documentation when adding new features
/// # Struct for storing all the information of each model.
///
/// <b> <u> The struct will store the following information: </u> </b>
///
/// * VertexBufferAny (Vertex data such as positions, normals, and tex coords)
/// * If it has texture (texture information such as path to texture file) or None to indicate that the model has no texture.
/// * If it has material (material information such as diffuse color, ambient color, specular color, etc...) or None to indicate that the model has no material.
pub struct Model{
   pub vertex : glium::vertex::VertexBufferAny,
   pub texture : Option<super::texture::Texture>,
   pub material : Option<super::material::Material>
}