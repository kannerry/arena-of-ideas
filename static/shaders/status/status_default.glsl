#include <common.glsl>

#ifdef VERTEX_SHADER
out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
void main() {
    v_quad_pos = a_pos * (1.0 + u_padding);
    float size = u_unit_radius * u_spawn;
    vec2 pos = v_quad_pos * size + u_unit_position;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_previous_texture;
in vec2 v_quad_pos;

const float c_status_radius_delta = .2;
const float c_status_radius_delta_max = .75;
const float c_status_radius_offset = .3;
const float c_status_thickness = .025;
const float c_status_dot_radius = 0.09;

vec4 renderStatusRing(vec2 uv, vec3 col)
{
    // return vec4(col, float(length(uv) > 1) * 1);
    float offset = 1. + c_status_radius_offset + c_status_radius_delta * u_status_index
        * (min(1., c_status_radius_delta_max / c_status_radius_delta / u_status_count));
    float rad = abs(vecAngle(uv) - pi);
    float h = abs(distance(uv,vec2(0.)) - offset);
    float dotDistance = distance(uv, vec2(0,-1) * offset);
    return vec4(col, 
        float(h < c_status_thickness && (u_status_duration == 1. || rad < u_status_time / u_status_duration * pi)
        || dotDistance < c_status_dot_radius));
}

void main() {
    commonInit();
    vec2 uv = v_quad_pos;
    vec4 previous_color = texture(u_previous_texture, gl_FragCoord.xy / vec2(textureSize(u_previous_texture, 0)));
    // float dist = distance(uv, vec2(0));
    // gl_FragColor = alphaBlend(previous_color, statusTint * float(dist < u_unit_radius));
    float t = u_time;
    vec4 col = vec4(0,0,0,0);
    // for (int i = 0; i < p_count; i++)
    //     col = alphaBlend(col, p_renderParticle(i + u_status_index * 100 + u_status_count * 333, uv, t));
    gl_FragColor = alphaBlend(previous_color, alphaBlend(col, renderStatusRing(uv, getColor().rgb)));
}
#endif