#include <common.glsl>
#ifdef VERTEX_SHADER
out vec2 uv;
attribute vec2 a_pos;

uniform vec2 u_align = vec2(0);

void main() {
    init_fields();
    uv = get_uv(a_pos);
    position += u_align * size;
    gl_Position = get_gl_position(uv);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 uv;

void main() {
    gl_FragColor = u_color;
}
#endif