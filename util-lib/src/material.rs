//reminder: need to include documentation when adding new features
/// # Struct for storing all the material data
/// The struct will store the following information:
///
/// * diffuse color ([f32;3])
/// * ambient color ([f32;3])
/// * specular color ([f32;3])
/// * shininess (f32)
/// * dissolve (f32)
/// * optical_density (f32)
#[derive(Default, Copy, Clone, Debug)]
pub struct Material{
   pub diffuse : [f32; 3],
   pub ambient : [f32; 3],
   pub specular : [f32; 3],
   pub shininess : f32,
   pub dissolve : f32,
   pub optical_density : f32,
}