#version 330
out vec4 fragColor;
in vec2 aTexCoords;
uniform sampler2D texData;

void main() {
	fragColor = texture(texData, aTexCoords);
}