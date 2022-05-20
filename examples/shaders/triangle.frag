#version 330 core

out vec4 colour;
uniform vec3 myColour;

void main() {
    colour = vec4(myColour.x, myColour.y, myColour.z, 1.0);
}
