#version 330
out vec4 fragColor;
in vec2 aTexCoords;
uniform sampler2D texData;
uniform vec3 backColour;

void main() {
	fragColor = vec4(backColour, 1.0) * texture(texData, aTexCoords);
}