#version 330

in vec2 fragment_uv;
out vec4 color;
uniform float glowEffect;
uniform sampler2D sampler;
uniform sampler2D rockSampler;
uniform sampler2D snowSampler;
uniform float time;
uniform float value;
uniform vec2 screen_size;
in vec4 positionCoord;

void main() {
	vec4 snowRockMixedTex = mix(texture(rockSampler, fragment_uv), texture(snowSampler, fragment_uv), clamp(1 - positionCoord.y + sin(time), 0.0, 1.0));
	vec4 mixedTex = mix(texture(sampler, fragment_uv), snowRockMixedTex, clamp((3-positionCoord.y + 1) / 3.0, 0.0, 1.0));
    color = mixedTex * vec4(glowEffect, glowEffect, glowEffect, 1.0);
}