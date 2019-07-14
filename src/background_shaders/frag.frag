#version 330
out vec4 fragColor;
in vec3 aColor;
uniform sampler2D texData;

void main() {
	fragColor = vec4(aColor, 1.0);
}