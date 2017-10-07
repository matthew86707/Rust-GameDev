#version 140

uniform vec2 movement;

in vec2 position;
in vec2 uv;
out vec2 fragment_uv;

void main() {
    fragment_uv = uv;
    gl_Position = vec4((position + movement) / 2, 0.0, 1.0);
}