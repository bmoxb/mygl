#version 330 core

out vec4 colour;

in vec2 textureCoord;

uniform sampler2D myTexture;

void main() {
    colour = texture(myTexture, textureCoord);
}
