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
in vec3 norm;

void main() {

// Get a lighting direction vector from the light to the vertex.
    vec3 lightVector = normalize(vec3(0.0, 5.0, 0.0) - positionCoord.xyz);
 
    // Calculate the dot product of the light vector and vertex normal. If the normal and light vector are
    // pointing in the same direction then it will get max illumination.
    float diffuse = clamp(max(dot(norm, lightVector), 0.1) / 10, 0.15, 1.2); 

	vec4 snowRockMixedTex = mix(texture(rockSampler, fragment_uv), texture(snowSampler, fragment_uv), clamp(1.5 - positionCoord.y + sin(time) * 5, 0.0, 1.0));
	vec4 mixedTex = mix(texture(sampler, fragment_uv), snowRockMixedTex, clamp((3-positionCoord.y + 1) / 3.0, 0.0, 1.0));
    color = vec4(diffuse, diffuse, diffuse, 1.0) * mixedTex * vec4(glowEffect, glowEffect, glowEffect, 1.0) * vec4(0.5, 0.5, 0.5, 1.0);
}