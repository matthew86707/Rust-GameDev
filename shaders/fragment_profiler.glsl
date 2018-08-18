#version 140

in vec2 fragment_uv;
out vec4 color;

uniform float time_passed;
uniform float mouseX;
uniform float mouseY;
uniform float zoom_uniform;

uniform float x;
uniform float y;

void main() {

    bool isShaded = false;

    float uv_x = fragment_uv.x;
    float uv_y = fragment_uv.y;

    float dx = x - uv_x;
    float dy = y - uv_y;
    if(sqrt((dx * dx) + (dy * dy)) < 0.1){
        isShaded = true;
    }

    if(isShaded){
          color = vec4(1.0, 1.0, 1.0, 1.0);
    }else{
        color = vec4(0.0, 0.0, 0.0, 0.0);
    }
}