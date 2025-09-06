#version 330 core

in vec3 fragPos;
in vec3 normal;

uniform vec3 lightPos;        // Posición de la luz (mundo)
uniform vec3 lightColor;      // Color de la luz
uniform vec3 viewPos;         // Posición de la cámara (mundo)
uniform vec3 objectColor;     // Color base del objeto
uniform float ambientStrength; // Fuerza de luz ambiente
uniform float specularStrength; // Fuerza del especular
uniform float shininess;       // Brillo del especular

out vec4 finalColor;

void main() {
    vec3 N = normalize(normal);
    vec3 L = normalize(lightPos - fragPos);

    // Difuso
    float diff = max(dot(N, L), 0.0);

    // Especular (Phong)
    vec3 V = normalize(viewPos - fragPos);
    vec3 R = reflect(-L, N);
    float spec = pow(max(dot(V, R), 0.0), shininess);

    vec3 ambient = ambientStrength * lightColor;
    vec3 diffuse = diff * lightColor;
    vec3 specular = specularStrength * spec * lightColor;

    vec3 color = (ambient + diffuse + specular) * objectColor;
    finalColor = vec4(color, 1.0);
}
