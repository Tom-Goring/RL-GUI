#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 i_pos;
layout(location=2) in vec3 i_color;
layout(location=3) in vec2 i_size;

layout (set = 0, binding = 0) uniform Globals {
    mat4 u_coord_translator;
    float u_scale;
};

layout(location=1) out vec4 v_color;

void main() {
    vec2 p_pos = i_pos * u_scale;
    vec2 p_scale = i_size * u_scale;

    mat4 transform = mat4(
        vec4(p_scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4(0.0, p_scale.y + 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(p_pos - vec2(0.5, 0.5), 0.0, 1.0)
    );

    v_color = vec4(i_color, 1.0);
//    vec4 tmp = transform * vec4(a_position.x, a_position.y, 0.0, 1.0);
//    gl_Position = vec4(tmp.x + i_pos.x * u_scale, tmp.y + i_pos.y, 0.0, 1.0);
    gl_Position = u_coord_translator * transform * vec4(a_position, 0.0, 1.0);
}
