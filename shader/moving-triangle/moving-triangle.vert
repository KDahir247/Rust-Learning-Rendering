#version 140

in vec3 position;

uniform float t;

void main() {

    vec3 pos = position;
    pos.x += t;
    gl_Position = vec4(pos, 1.0);
}