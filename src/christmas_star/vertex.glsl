#version 430

layout (location = 0) in vec4 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec4 diffuse_color;

out vec4 color;

void main() {
    // float intensity = max(dot(n, l_dir), 0.0);
    // DataOut.color = 
    color = diffuse_color;
    gl_Position = position;
}
