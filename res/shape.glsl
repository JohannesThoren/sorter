#shader vertex
#version 330 core

layout(location = 0) in vec4 v_bounds;
layout(location = 1) in int v_color;

out int tight_color;

void main() {
    gl_Position = v_bounds;
    tight_color = v_color;
}

#shader geometry
#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 128) out;

in int tight_color[];

out vec4 color;

const int DRAW_RECT = 0;
const int DRAW_TRIANGLE = 1;
const int DRAW_ELLIPSE = 2;
const int DRAW_LINE = 3;

uniform int u_primitive = 0;
uniform int u_ellipse_detail = 100;
uniform float u_line_width = 0.01;


vec4 normal_color(int tight_color) {
    float a = tight_color & 255;
    float b = (tight_color >> 8) & 255;
    float g = (tight_color >> 16) & 255;
    float r = (tight_color >> 24) & 255;
    return vec4(r / 255, g / 255, b / 255, a / 255);
}

void draw_rect(float x, float y, float width, float height) {
    //(0, 0)
    gl_Position = vec4(x, y, 1.0, 1.0);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(x + width, y, 1.0, 1.0);
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(x, y + height, 1.0, 1.0);
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(x + width, y + height, 1.0, 1.0);
    EmitVertex();
}

void draw_triangle(float x, float y, float width, float height) {
    //(0.5, 0)
    gl_Position = vec4(x + width / 2.0, y + height, 1.0, 1.0);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(x + width, y, 1.0, 1.0);
    EmitVertex();

    //(0, 0)
    gl_Position = vec4(x, y, 1.0, 1.0);
    EmitVertex();
}

void draw_ellipse(float x, float y, float width, float height) {
    for(int i = 0; i < u_ellipse_detail+1; i++) {
        float nx = cos(float(i) / float(u_ellipse_detail) * 2.0 * 3.14) * width / 2.0;
        float ny = sin(float(i) / float(u_ellipse_detail) * 2.0 * 3.14) * height / 2.0;
        nx = x + width / 2.0 + nx;
        ny = y + height / 2.0 + ny;

        gl_Position = vec4(nx, ny, 1.0, 1.0);
        EmitVertex();

        gl_Position = vec4(x + width / 2.0, y + height / 2.0, 1.0, 1.0);
        EmitVertex();
    }
    float nx = cos(float(0) / float(u_ellipse_detail) * 2.0 * 3.14) * width / 2.0;
    float ny = sin(float(0) / float(u_ellipse_detail) * 2.0 * 3.14) * height / 2.0;
    nx = x + width / 2.0 + nx;
    ny = y + height / 2.0 + ny;

    gl_Position = vec4(nx, ny, 1.0, 1.0);
    EmitVertex();

    gl_Position = vec4(x + width / 2.0, y + height / 2.0, 1.0, 1.0);
    EmitVertex();
}

void draw_line(float x1, float y1, float x2, float y2) {
    float x_len = x1 - x2;
    float y_len = y1 - y2;
    
    float a = atan(y_len / x_len);
    float pi = 3.14 / 2.0;

    vec2 pos1 = vec2(x1 + cos(a + pi) * u_line_width, y1 + sin(a + pi) * u_line_width); //(0, 0)
    vec2 pos2 = vec2(x1 + cos(a - pi) * u_line_width, y1 + sin(a - pi) * u_line_width); //(1, 0)
    vec2 pos3 = vec2(x2 + cos(a + pi) * u_line_width, y2 + sin(a + pi) * u_line_width); //(0, 1)
    vec2 pos4 = vec2(x2 + cos(a - pi) * u_line_width, y2 + sin(a - pi) * u_line_width); //(1, 1)

    ///////////////////////////////////////////////////////

    //(0, 0)
    gl_Position = vec4(pos1.x, pos1.y, 1.0, 1.0);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(pos2.x, pos2.y, 1.0, 1.0);
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(pos3.x, pos3.y, 1.0, 1.0);
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(pos4.x, pos4.y, 1.0, 1.0);
    EmitVertex();
}

void main() {
    vec4 col = normal_color(tight_color[0]);
    vec4 pos = gl_in[0].gl_Position;

    color = col;

    ///////////////////////////////////////////////////////
    if(u_primitive == DRAW_ELLIPSE) {
        draw_ellipse(pos.x, pos.y, pos.z, pos.w);
    }
    else if(u_primitive == DRAW_LINE) {
        draw_line(pos.x, pos.y, pos.z, pos.w);
    }
    else if(u_primitive == DRAW_RECT) {
        draw_rect(pos.x, pos.y, pos.z, pos.w);
    }
    else if(u_primitive == DRAW_TRIANGLE) {
        draw_triangle(pos.x, pos.y, pos.z, pos.w);
    }
   
}

#shader fragment
#version 330 core

in vec4 color;


layout(location = 0) out vec4 out_color;


void main() {
	out_color = color;
}