//reminder: need to include documentation when adding new features/// # Struct for storing the texture file name.
/// # Contains parameters for all the texture file name that can be parsed from wavefront obj files.
///  <b> <u> List of textures from wavefront obj files: </u> </b>
///
/// * diffuse texture
/// * ambient texture
/// * specular texture
/// * shininess texture
/// * normal texture
/// * dissolve texture

#[derive(Default, Clone, Debug)]
pub struct Texture{
   pub diffuse_texture : String,
   pub ambient_texture : String,
   pub specular_texture : String,
   pub shininess_texture : String,
   pub normal_texture : String,
   pub dissolve_texture : String,
}