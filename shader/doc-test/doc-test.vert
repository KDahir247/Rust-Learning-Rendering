#version 140

in vec2 position;

uniform float t;
uniform float v;

void main() {

    vec2 pos = position;
    pos.x += t;
    pos.y += v;
    gl_Position = vec4(pos,0.0,1.0);
}