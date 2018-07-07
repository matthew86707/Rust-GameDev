#version 140

in vec2 fragment_uv;
out vec4 color;

uniform float time_passed;

void main() {
    float x = fragment_uv.x + -0.4;
    float y = fragment_uv.y + 0.4;
    float isShaded = 0.0;
    float isShadedCircle = (pow(x - 0.5, 2) + pow(y - 0.5, 2)) + 0.0001;
    float isShadedEllipse = (pow(x - 0.5, 2) + pow(y - 0.45, 2) / 10.0) + 0.0001;
    isShaded = min(isShadedCircle, isShadedEllipse);
  
    color = vec4(1.0 - clamp((isShaded) * 100.0, 0.0, 1.0), 0.0, 0.0, 1.0);
 
}