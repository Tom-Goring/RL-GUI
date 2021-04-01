#version 450

layout(location=0) in vec4 input_color;
layout(location=1) in vec2 input_position;
layout(location=2) in vec2 input_size;
layout(location=3) in vec3 input_border_colour;
layout(location=4) in float input_border_width;

layout(location=0) out vec4 output_color;

void main() {
    vec2 top_left = input_position;
    vec2 bottom_right = input_position + input_size;

    vec4 pixel = gl_FragCoord;

    if (pixel.x > bottom_right.x - input_border_width ||
        pixel.x < top_left.x + input_border_width - 1.0 || // not sure why this extra pixel is needed
        pixel.y < top_left.y + input_border_width - 1.0 ||
        pixel.y > bottom_right.y - input_border_width ||
        pixel.y < 1) {
        output_color = vec4(input_border_colour, 1.0);
    } else {
        output_color = input_color;
    }

//    float colour = input_border_width / 100;
//
//    output_color = vec4(input_border_width, 0.0, 0.0, 1.0);


}
