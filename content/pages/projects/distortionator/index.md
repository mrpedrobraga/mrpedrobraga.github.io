---
title: Distortionator (2.0.0)
description: A very niche battle-background editor for Godot-based, MOTHER-like games.
tags:
  - software
  - graphics
  - tool
link: https://mrpedrobraga.itch.io/modot-distortionator
banner_url: https://dreamtonegame.com/Resources/Assets/Screenshots/screen5.png
---
<center><a href="https://github.com/mrpedrobraga/modot-distortionator"><img src="https://img.shields.io/badge/github-repo-blue?logo=github"></a></center>

<iframe width="560" height="315" src="https://www.youtube.com/embed/0OEWLVnX30A?si=x_FAhQWqeLHrCZOw" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

## How to use
Install the [Integration Addon](https://github.com/mrpedrobraga/modot-distortionator/tree/3.x/addons/distortionator_integration) (in `/addons`) in your Godot project. This addon contains all the necessary code to import `.dsp` files and visualise the battle background within your Godot game.

Then, in Distortionator, make sure to open your Godot Project's folder (the one containing a `project.godot` file). You can use any textures in the Godot Project in Distortionator.

Here is an example `.dsp` file — it's just all the settings stored as [INI](https://en.wikipedia.org/wiki/INI_file).

```ini
[]

shader_folder="res://"
texture_folder="res://"

[Layer 0]

shader="[DEFAULT]"
texture="[DEFAULT]"
screen_size=Vector2( 320, 180 )
move=Vector2( 0.2, 0.2 )
ping_pong_speed=Vector2( 0, 0 )
oscillation_amplitude=Vector2( 0, 0 )
oscillation_frequency=Vector2( 0, 0 )
oscillation_speed=Vector2( 0, 0 )
osc_amp_ping_pong=Vector2( 0, 0 )
osc_trans_ping_pong=Vector2( 0, 0 )
compression_amplitude=Vector2( 0, 0 )
compression_frequency=Vector2( 0, 0 )
compression_speed=Vector2( 0, 0 )
comp_amp_ping_pong=Vector2( 0, 0 )
comp_trans_ping_pong=Vector2( 0, 0 )
interlaced_amplitude=Vector2( 0, 0 )
interlaced_frequency=Vector2( 0, 0 )
interlaced_speed=Vector2( 0, 0 )
inter_amp_ping_pong=Vector2( 0, 0 )
inter_trans_ping_pong=Vector2( 0, 0 )
palette_shifting_speed=0.0
palette="[Resource]/test/bbg_blue_star.png"
palette_shifting=false
barrel=false
effect=1.0
effect_scale=2.0
barrelxy=Vector2( 1, 1 )
```
## Building from source
It's a Godot project, just run it man...
## FAQ
#### Why not a Godot addon?
This editor was created so you could edit `.dsp`s on [Mother Encore](https://motherencore.com/), which is a large Godot project that takes a while to startup. Having a separate editor allows artists to not even touch Godot.
#### Will this receive any updates in the future?
I'm attempting to port Distortionator to Godot 4.x. All features made with this version will still work with Godot 3.x, but they'll use different integration addons. I hope to do a little revamping of the interface, making it more modern and fun to use!
## Alternatives
If you don't want to use `.dsp` files, you can easily write your own battle backgrounds by adding `ShaderMaterial`s to `TextureRect`s...

This is the exact shader used by Distortionator. If you study it, you'll understand how to create and modify all the different effects.

```glsl
shader_type canvas_item;
// replace \"blend_mix\" with
// \"blend_add\" or \"blend_sub\" or \"blend_mul\"
// to change blend mode
render_mode blend_mix;

uniform float opacity = 1.0;
uniform vec2 screen_size = vec2(320, 180);

// SCROLL
uniform vec2 move = vec2(0,0);

// PING PONG
uniform vec2 ping_pong_speed = vec2(0,0);

// OSCILLATION
uniform vec2 oscillation_amplitude = vec2(0,0);
uniform vec2 oscillation_frequency = vec2(0,0);
uniform vec2 oscillation_speed = vec2(0,0);
uniform vec2 osc_amp_ping_pong = vec2(0,0);
uniform vec2 osc_trans_ping_pong = vec2(0,0);

// COMPRESSION
uniform vec2 compression_amplitude = vec2(0,0);
uniform vec2 compression_frequency = vec2(0,0);
uniform vec2 compression_speed = vec2(0,0);
uniform vec2 comp_amp_ping_pong = vec2(0,0);
uniform vec2 comp_trans_ping_pong = vec2(0,0);

// INTERLACING
uniform vec2 interlaced_amplitude = vec2(0,0);
uniform vec2 interlaced_frequency = vec2(0,0);
uniform vec2 interlaced_speed = vec2(0,0);
uniform vec2 inter_amp_ping_pong = vec2(0,0);
uniform vec2 inter_trans_ping_pong = vec2(0,0);

// PALETTE SHIFTING
uniform float palette_shifting_speed = 0;
uniform sampler2D palette;
uniform bool palette_shifting;
uniform int palette_anim_frame_count;

// BARREL
uniform bool barrel = false;
uniform float effect = 1; // -1.0 is BARREL, 0.1 is PINCUSHION. For planets, ideally -1.1 to -4.
uniform float effect_scale = 2; // Play with this to slightly vary the results.
uniform vec2 barrelxy = vec2(1.0,1.0);

vec2 distort(vec2 p) {
	float d = length(p);
	float z = sqrt(1.0 + d * d * effect);
	float r = atan(d, z) / 3.14159;
	r *= effect_scale;
	float phi = atan(p.y, p.x);
	return vec2(r*cos(phi)+.5,r*sin(phi)+.5);
}

 void fragment(){
	//UV variable for distortions
	vec2 newuv = UV;
	
	//Original modulate
	vec4 modulate = COLOR;
	
	//BARREL
	if (barrel){
		vec2 xy = vec2(2.0 * UV) - barrelxy;
		newuv = distort(xy);
	} else {
		newuv = UV;
	}
	
	// OSCILATION
	vec2 osc_time = vec2(TIME, TIME);
	if (osc_trans_ping_pong.x != 0.0) {
		osc_time.x = cos(osc_trans_ping_pong.x * TIME);
	}
	if (osc_trans_ping_pong.y != 0.0) {
		osc_time.y = cos(osc_trans_ping_pong.y * TIME);
	}
	
	if ((oscillation_frequency.x != 0.0) && (oscillation_amplitude.x != 0.0)) { //horizontal oscillation
		newuv.x += oscillation_amplitude.x * cos((oscillation_frequency.x * newuv.y) + osc_time.x * oscillation_speed.x) * cos(osc_amp_ping_pong.x * TIME);
	}
	if ((oscillation_frequency.y != 0.0) && (oscillation_amplitude.y != 0.0)) { //vertical oscillation
		newuv.y += oscillation_amplitude.y * cos((oscillation_frequency.y * newuv.x) + osc_time.y * oscillation_speed.y) * cos(osc_amp_ping_pong.y * TIME);
	}
	
	// COMPRESSION
	vec2 comp_time = vec2(TIME, TIME);
	if (comp_trans_ping_pong.x != 0.0) {
		comp_time.x = cos(comp_trans_ping_pong.x * TIME);
		}
	if (comp_trans_ping_pong.y != 0.0) {
		comp_time.y = cos(comp_trans_ping_pong.y * TIME);
		}
	
	if ((compression_frequency.x != 0.0) && (compression_amplitude.x != 0.0)) { //horizontal compression
		newuv.x += compression_amplitude.x * cos((compression_frequency.x * newuv.x) + comp_time.x * compression_speed.x) * cos(comp_amp_ping_pong.x * TIME);
	}
	if ((compression_frequency.y != 0.0) && (compression_amplitude.y != 0.0)) { //vertical compression
		newuv.y += compression_amplitude.y * cos((compression_frequency.y * newuv.y) + comp_time.y * compression_speed.y) * cos(comp_amp_ping_pong.y * TIME);
	}
	
	// PING PONG!
	if (ping_pong_speed.x != 0.0) {
		newuv.x += move.x * cos(ping_pong_speed.x * TIME);
	} else {
		newuv.x += TIME * move.x/0.5;
	}
	if (ping_pong_speed.y != 0.0) {
		newuv.y += move.y * cos(ping_pong_speed.y * TIME);
	} else {
		newuv.y += TIME * move.y/0.5;
	}
	
	// INTERLACING
	float diff_x = 0.0;
	float diff_y = 0.0;
	
	vec2 inter_time = vec2(TIME, TIME);
	if (inter_trans_ping_pong.x != 0.0) {
			inter_time.x = cos(inter_trans_ping_pong.x * TIME);
		}
	if (inter_trans_ping_pong.y != 0.0) {
			inter_time.y = cos(inter_trans_ping_pong.y * TIME);
		}
	
	// INTERLACING (X)
	if ((interlaced_frequency.x != 0.0) && (interlaced_amplitude.x != 0.0)) {
		if ( int(UV.y * screen_size.y / 2.0)  % 2 == 0 ){
			diff_x += 0.05 * cos((interlaced_frequency.x * UV.y) + (interlaced_speed.x * inter_time.x)) * interlaced_amplitude.x * cos(inter_amp_ping_pong.x * TIME);
		} else{
			diff_x -= 0.05 * cos((interlaced_frequency.x * UV.y) + (interlaced_speed.x * inter_time.x)) * interlaced_amplitude.x * cos(inter_amp_ping_pong.x * TIME);
		}
	}
	
	// INTERLACING (Y)
	if ((interlaced_frequency.y != 0.0) && (interlaced_amplitude.y != 0.0)) {
		if ( int(UV.x * screen_size.x / 4.0)  % 2 == 0 ){
			diff_y += 0.05 * cos((interlaced_frequency.y * UV.x) + (interlaced_speed.y * inter_time.y)) * interlaced_amplitude.y * cos(inter_amp_ping_pong.y * TIME);
		} else{
			diff_y -= 0.05 * cos((interlaced_frequency.y * UV.x) + (interlaced_speed.y * inter_time.y)) * interlaced_amplitude.y * cos(inter_amp_ping_pong.y * TIME);
		}
	}
	
	// Apply all distortions!
	COLOR = (textureLod(TEXTURE, vec2(newuv.x + diff_x, newuv.y + diff_y), 0));
	
	// PALETTE SHIFTING
	if (palette_shifting) {
		COLOR = vec4(texture(palette, vec2(COLOR.r, mod(-TIME * palette_shifting_speed * 1.0 / float(palette_anim_frame_count), 1.0))).rgb, COLOR.a);
	}
	
	// OPACITY
	COLOR.a *= opacity;
}
```