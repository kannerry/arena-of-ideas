#include <common.glsl>

#ifdef VERTEX_SHADER
out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;

void main() {
    v_quad_pos = a_pos * 100.0;
    vec2 pos = v_quad_pos * 1.0;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 v_quad_pos;
uniform vec4 u_color_1;

void main() {
    vec4 color = mix(u_color, u_color_1, float(v_quad_pos.x < .0));
    gl_FragColor = vec4(1);
}
#endif
