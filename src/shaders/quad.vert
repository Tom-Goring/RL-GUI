#version 450

// for each instance, we go through each vertex of the original vertices and calculate the instance's vertices

layout(location=0) in vec2 original_vertex; // origin square
layout(location=1) in vec2 instance_vertex; // position to move origin square to
layout(location=2) in vec3 instance_colour; // color to make origin square - passed to frag shader
layout(location=3) in vec2 instance_size; // size to scale origin square to
layout(location=4) in vec3 instance_border_colour; // color to make border - passed to frag shader
layout(location=5) in float instance_border_width;

layout (set = 0, binding = 0) uniform Globals {
    mat4 u_coord_translator;
    float u_scale;
};

layout(location=0) out vec4 output_color;
layout(location=1) out vec2 output_position;
layout(location=2) out vec2 output_size;
layout(location=3) out vec3 output_border_colour;
layout(location=4) out float output_border_width;

void main() {
    vec2 scaled_position = instance_vertex * u_scale;
    vec2 scaled_size = instance_size * u_scale;

    mat4 transform = mat4(
        vec4(scaled_size.x + 1.0, 0.0, 0.0, 0.0),
        vec4(0.0, scaled_size.y + 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(scaled_position - vec2(0.5, 0.5), 0.0, 1.0)
    );

    output_color = vec4(instance_colour, 1.0);
    output_position = scaled_position;
    output_size = scaled_size;
    output_border_colour = instance_border_colour;
    output_border_width = instance_border_width;

    // rounded borders in the future? border radius?

    vec4 vertex_position = u_coord_translator * transform * vec4(original_vertex, 0.0, 1.0);
    gl_Position = vertex_position;
}
