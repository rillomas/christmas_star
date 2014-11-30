#version 430

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec4 diffuse_color;

uniform vec3 direction_to_light;

out vec4 color;

void main() {
    float intensity = max(dot(normal, direction_to_light), 0.0);
    color = intensity * diffuse_color;
    gl_Position = vec4(position,1.0);
}
