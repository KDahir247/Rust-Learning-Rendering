#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

in vec2 tex_coord;
out vec2 v_tex_coord;

uniform mat4 matrix;
uniform mat4 view;
uniform mat4 perspective;

void main() {

    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = perspective * view * matrix * vec4(position,1.0);

    v_position = gl_Position.xyz / gl_Position.w;
    v_tex_coord = tex_coord;
}