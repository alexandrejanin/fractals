#version 330 core
out vec4 fragColor;
in vec4 gl_FragCoord;

uniform vec2 resolution;

float sphereSDF(vec3 point, vec3 center, float radius) {
    return length(point - center) - radius;
}

float SDF(vec3 point) {
    return min(
    sphereSDF(point, vec3(-0.5, 0, 0), 1),
    sphereSDF(point, vec3(1, 0, 0), 0.5)
    );
}

float castRay(vec3 rayOrigin, vec3 rayDir) {
    float dist = 0;
    float minDist = 100000;

    for (int i = 0; i < 100; i++) {
        float res = SDF(rayOrigin + rayDir * dist);
        if (res < (0.0001*dist)) {
            return dist;
        }
        dist += res;
    }

    return -1.0;
}

vec3 calcNormal(vec3 pos) {
    float c = SDF(pos);
    vec2 eps_zero = vec2(0.001, 0.0);
    return normalize(vec3(SDF(pos + eps_zero.xyy), SDF(pos + eps_zero.yxy), SDF(pos + eps_zero.yyx)) - c);
}

vec3 render(vec3 rayOrigin, vec3 rayDir) {
    float dist = 0;
    float minDist = 100000;

    for (int i = 0; i < 100; i++) {
        float res = SDF(rayOrigin + rayDir * dist);
        if (res < 0.0001 * dist) {
            vec3 normal = calcNormal(rayOrigin + dist * rayDir);
            vec3 col = normal * vec3(0.5) + vec3(0.5);

            return col;
        }

        if (res < minDist)
        minDist = res;

        dist += res;
    }

    vec3 skybox = vec3(0.30, 0.36, 0.60) - (rayDir.y * 0.7);
    vec3 glow = vec3(clamp(1 - (7 * minDist), 0, 1));

    return 0.8 * skybox + 0.2 * glow;
}

vec3 getCameraRayDir(vec2 uv, vec3 camPos, vec3 camTarget) {
    vec3 camForward = normalize(camTarget - camPos);
    vec3 camRight = normalize(cross(vec3(0.0, 1.0, 0.0), camForward));
    vec3 camUp = normalize(cross(camForward, camRight));

    float fPersp = 2.0;
    vec3 vDir = normalize(uv.x * camRight + uv.y * camUp + camForward * fPersp);

    return vDir;
}

vec2 normalizeScreenCoords(vec2 screenCoord) {
    vec2 result = 2.0 * (screenCoord/resolution.xy - 0.5);
    result.x *= resolution.x/resolution.y;
    return result;
}

void main() {
    vec3 camPos = vec3(0, 0, -5);
    vec3 camTarget = vec3(0, 0, 0);

    vec2 coords = normalizeScreenCoords(gl_FragCoord.xy);
    vec3 rayDir = getCameraRayDir(coords, camPos, camTarget);

    vec3 col = render(camPos, rayDir);

    fragColor = vec4(col, 1);
}
