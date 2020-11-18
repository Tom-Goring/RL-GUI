#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_color;
layout(location=2) in vec2 i_pos;
layout(location=3) in vec3 i_color;

layout(location=1) out vec4 v_color;


void main() {
    v_color = vec4(i_color, 1.0);
    gl_Position =  vec4(a_position.x + i_pos.x, a_position.y + i_pos.y, 0.0, 1.0);
}