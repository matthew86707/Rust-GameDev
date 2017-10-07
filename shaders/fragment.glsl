#version 140

in vec2 fragment_uv;
out vec4 color;

uniform sampler2D sampler;
uniform float value;
uniform vec2 screen_size;

void main() {
	//vec2 uv = gl_FragCoord.xy / screen_size;
	//vec2 uv;
	//uv.x = sin(gl_FragCoord.x / screen_size.x) / 2 + fragment_uv.x;
	//uv.y = sin(gl_FragCoord.y / screen_size.y) / 2 + fragment_uv.y;
	
    color = texture(sampler, fragment_uv);
}