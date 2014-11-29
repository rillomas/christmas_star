#version 430

in vec4 color;
out vec4 pix_color;
void main() {
    // pix_color = vec4(1.0, 0.0, 0.0, 1.0);
    pix_color = color;
}