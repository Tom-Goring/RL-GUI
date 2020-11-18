#version 450

struct Quad {
    vec2 pos;
    vec2 size;
    vec4 color;
};

layout(set=0, binding=0) buffer Vertices {
    Quad quads[];
};

layout(location=0) out vec4 v_color;

void main() {
    v_color = quads[gl_InstanceIndex].color;
    gl_Position = vec4(quads[gl_InstanceIndex].pos, 0.0, 1.0);
}
