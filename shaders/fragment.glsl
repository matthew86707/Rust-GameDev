#version 330

in vec2 fragment_uv;
in float shading_i;
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

    vec3 lightVector = normalize(-(vec3(35.0, 800.0, 800.0) - positionCoord.xyz));
    float diffuse = clamp(max(dot(norm, lightVector), 0.1) / 10, 0.1, 1.2); 

	vec4 snowRockMixedTex = mix(texture(rockSampler, fragment_uv), texture(snowSampler, fragment_uv), clamp(1.5 - positionCoord.y + sin(time) * 5, 0.0, 1.0));
	vec4 mixedTex = mix(texture(sampler, fragment_uv), snowRockMixedTex, clamp((3-positionCoord.y + 1) / 3.0, 0.0, 1.0));
	if(shading_i == 0.0){
		color =  mixedTex * vec4(glowEffect, glowEffect, glowEffect, 1.0) * vec4(0.5, 0.5, 0.5, 1.0);
	}else{
    	color = vec4(diffuse, diffuse, diffuse, 1.0) * mixedTex * vec4(glowEffect, glowEffect, glowEffect, 1.0) * vec4(0.5, 0.5, 0.5, 1.0);
    }
}