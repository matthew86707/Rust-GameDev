#version 330

out vec4 color;
uniform sampler2D sampler;
in vec2 fragment_uv;
uniform float time;

void main() {
    color = vec4(1.0, 1.0, 1.0, 0.5) * mix(vec4(0.0, 0.0, 1.0, 0.2), texture(sampler, fragment_uv * 150), 0.5);
    //color = vec4(1.0, 1.0, 1.0, 1.0);
}