#version 430

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 diffuse_color;

uniform mat4 mvp;
out vec4 color;

void main() {
    color = diffuse_color;
    gl_Position = mvp * vec4(position,1.0);
}
