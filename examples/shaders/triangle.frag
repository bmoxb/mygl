#version 330 core
out vec4 colour;
void main() {
    colour = vec4(colour.x, colour.y, colour.z, 1.0);
}
