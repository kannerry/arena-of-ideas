#include <common.glsl>

#ifdef VERTEX_SHADER
out vec2 uv;
attribute vec2 a_pos;
uniform int u_trail_count;

flat out int p_index;
flat out float p_t;

void main() {
    init_fields();
    int trail_index = gl_InstanceID % u_trail_count;
    float trail_shift = 0.002 * trail_index;
    p_index = gl_InstanceID - trail_index;
    p_t = u_t + trail_shift;
    vec2 vel = (rand_circle(p_index) + sin(rand_vec(p_index) * PI * 2 + p_t * 1.5)) * rand(p_index + 1) * 2.5;
    position += vel * p_t;
    uv = get_uv(a_pos);
    gl_Position = get_gl_position(uv);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 uv;
flat in int p_index;
flat in float p_t;

void main() {
    float dist = length(uv);
    if(dist > 1. - p_t)
        discard;
    vec3 color = mix(vec3(0), vec3(1), float(p_index % 2));
    gl_FragColor = vec4(color, 1.);
}
#endif
