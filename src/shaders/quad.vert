#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 i_pos;
layout(location=2) in vec3 i_color;
layout(location=3) in vec2 i_size;

layout(location=1) out vec4 v_color;

void main() {

    mat4 transform = mat4(
        vec4(i_size.x, 0.0, 0.0, 0.0),
        vec4(0.0, i_size.y, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    );

    v_color = vec4(i_color, 1.0);
    vec4 tmp = transform * vec4(a_position.x, a_position.y, 0.0, 1.0);
    gl_Position = vec4(tmp.x + i_pos.x, tmp.y + i_pos.y, 0.0, 1.0);
}
