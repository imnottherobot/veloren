#version 330 core

#include <globals.glsl>
#include <random.glsl>

in vec3 f_pos;
in vec3 f_chunk_pos;
flat in uint f_pos_norm;
in vec3 f_col;
in float f_light;
in float f_ao;

layout (std140)
uniform u_locals {
	vec3 model_offs;
	float load_time;
};

out vec4 tgt_color;

#include <sky.glsl>
#include <light.glsl>

float vmax(vec3 v) {
	return max(v.x, max(v.y, v.z));
}

void main() {
	// First 3 normals are negative, next 3 are positive
	vec3 normals[6] = vec3[](vec3(-1,0,0), vec3(1,0,0), vec3(0,-1,0), vec3(0,1,0), vec3(0,0,-1), vec3(0,0,1));

	// TODO: last 3 bits in v_pos_norm should be a number between 0 and 5, rather than 0-2 and a direction.
	uint norm_axis = (f_pos_norm >> 30) & 0x3u;
	// Increase array access by 3 to access positive values
	uint norm_dir = ((f_pos_norm >> 29) & 0x1u) * 3u;
	// Use an array to avoid conditional branching
	vec3 f_norm = normals[(f_pos_norm >> 29) & 0x7u];

	float ao = pow(f_ao, 0.5) * 0.9 + 0.1;

	vec3 light, diffuse_light, ambient_light;
	get_sun_diffuse(f_norm, time_of_day.x, light, diffuse_light, ambient_light, 1.0);
	float point_shadow = shadow_at(f_pos, f_norm);
	diffuse_light *= point_shadow;
	ambient_light *= point_shadow;
	vec3 point_light = light_at(f_pos, f_norm);
	light += point_light;
	ambient_light *= f_light * ao;
	diffuse_light *= f_light * ao;
	diffuse_light += point_light * ao;

	vec3 col = f_col + hash(vec4(floor(f_chunk_pos * 3.0 - f_norm * 0.5), 0)) * 0.02; // Small-scale noise

	// Select glowing
	if (select_pos.w > 0 && select_pos.xyz == floor(f_pos - f_norm * 0.01)) {
		if (vmax(abs(mod(f_pos - f_norm * 0.5, 1.0) - 0.5)) > 0.45) {
			col *= 0.5;
		}
	}

	vec3 surf_color = illuminate(srgb_to_linear(col), light, diffuse_light, ambient_light);

	float fog_level = fog(f_pos.xyz, focus_pos.xyz, medium.x);
	vec4 clouds;
	vec3 fog_color = get_sky_color(normalize(f_pos - cam_pos.xyz), time_of_day.x, cam_pos.xyz, f_pos, 1.0, true, clouds);
	vec3 color = mix(mix(surf_color, fog_color, fog_level), clouds.rgb, clouds.a);

	tgt_color = vec4(color, 1.0);
}
