#version 330

uniform vec4 color;
uniform vec2 offset;
uniform vec2 scale;
uniform vec2 dims;
uniform float rounding;
uniform float border_width;

in vec2 vert;

out vec4 out_color;

void main () {
    vec2 uv = gl_FragCoord.xy;
    vec2 top_left = offset * dims;
    vec2 bottom_right = top_left + scale * dims;
    if ((uv.x < top_left.x     || uv.x > (top_left.x + border_width)) &&
        (uv.x > bottom_right.x || uv.x < (bottom_right.x - border_width)) &&
        (uv.y < top_left.y     || uv.y > (top_left.y + border_width)) &&
        (uv.y > bottom_right.y || uv.y < (bottom_right.y - border_width))) {
        discard;
    }
    out_color = color;
}
