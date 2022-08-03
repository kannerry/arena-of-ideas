#include <common.glsl>


#ifdef VERTEX_SHADER
out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
void main() {
    // float u_action_time = u_time - fract(u_time);
    float action_t = smoothstep(ACTION_ANIMATION_TIME, 0, u_time - u_action_time);
    action_t *= action_t;
    v_quad_pos = a_pos * (1.0 + u_padding);
    float size = u_unit_radius + action_t * .3;
    vec2 pos = v_quad_pos * size + u_unit_position;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.0);
    gl_Position = vec4(p_pos.xy, 0.0, p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 v_quad_pos;

float getRingAlpha(
    vec2 uv, float r, float thickness, float glow, float glowStartV, float innerMult, float outerMult)
{
    float dist = distance(uv, vec2(0.));
    float circleDist = abs(r - dist);
    float halfThickness = thickness * .5;
    glow *= max(r, 0.5);
    return max(float(circleDist < halfThickness),

        float(circleDist > halfThickness && circleDist < halfThickness + glow)
        * mix(glowStartV, 0., (circleDist - halfThickness) / glow)
        * (float(dist > r) * outerMult + float(dist < r) * 1.)
        * (float(dist < r) * innerMult + float(dist > r) * 1.));
}

void main() {
    commonInit();
    float glow = 0.35 + sin(u_time) * .1;

    vec2 uv = v_quad_pos;

    vec3 colors[2];
    colors[0] = getColor().rgb;
    colors[1] = vec3(1, 0.980, 0.941);

    float innerTime = u_time - floor(u_time / pi * 2.) * pi * 2. + u_random * 100.;

    float outerR = 1., innerR = 0.8 + sin(innerTime) * .5;

    const float innerFade = 1.2;
    float innerAlpha = float(cos(innerTime) > 0.) + float(cos(innerTime + innerFade));
    innerAlpha = clamp(innerAlpha, 0., 1.);

    float innerAlpha2 = -1., innerR2 = -1.;

    float distCenter = distance(uv,vec2(0.0,0.0));
    float distOuter = outerR - distCenter;
    float distInner = innerR - distCenter;


    vec4 col = vec4(colors[0], getRingAlpha(uv, outerR, thicknessOuter, glow, .3, 1.5, 0.));
    col = alphaBlend(col, vec4(colors[0], getRingAlpha(uv, outerR, thicknessOuter, glow, .3, 1.5, 0.)));
    col = alphaBlend(col, vec4(colors[0], getRingAlpha(uv, innerR, 0., glow * 1.5, .5, 1.7, 1.) * innerAlpha));
    
    float insideCircTime = u_time + u_random * 200.;
    float v = mix(0.5, 0.0, distance(uv,
        vec2(cos(insideCircTime * 1.13 + sin(insideCircTime * .5) * 2.),
            sin(insideCircTime * 2.73)) * innerR * .8)) * float(distCenter < outerR + thicknessOuter * .5);
    col = alphaBlend(col, vec4(colors[0], v));
    if (length(uv) < outerR)
    {
        col *= col.a;
        col.a = 1;
    }
    gl_FragColor = col;
}
#endif