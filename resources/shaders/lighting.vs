#version 330 core

// Raylib attribute names
layout (location = 0) in vec3 vertexPosition;
layout (location = 2) in vec3 vertexNormal;

uniform mat4 mvp;
uniform mat4 matModel;

out vec3 fragPos;
out vec3 normal;

void main() {
    vec4 worldPos = matModel * vec4(vertexPosition, 1.0);
    fragPos = worldPos.xyz;

    mat3 normalMatrix = mat3(transpose(inverse(matModel)));
    normal = normalize(normalMatrix * vertexNormal);

    gl_Position = mvp * vec4(vertexPosition, 1.0);
}
