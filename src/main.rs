/*
*   Copyright (c) 2020 Johannes Thorén
*   All rights reserved.

*   Permission is hereby granted, free of charge, to any person obtaining a copy
*   of this software and associated documentation files (the "Software"), to deal
*   in the Software without restriction, including without limitation the rights
*   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
*   copies of the Software, and to permit persons to whom the Software is
*   furnished to do so, subject to the following conditions:

*   The above copyright notice and this permission notice shall be included in all
*   copies or substantial portions of the Software.

*   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
*   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
*   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
*   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
*   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
*   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
*   SOFTWARE.
*/

use engine_core::{self, info_log, window::Window};
use engine_gui::{
    comps::{Button, Slider},
    gui::GUI,
    gui::*,
};
use engine_renderer::{self, color::*, font::Font, graphics::Graphics, renderer::{init_gl, std_renderer::{BlendMode, blend_func}}, shader::Shader};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{thread, time};
use time::{Duration, SystemTime};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

#[cfg(target_os = "windows")]
const FONT_PATH: &str = "C:/Windows/Fonts/arial.ttf";

#[cfg(target_os = "linux")]
const FONT_PATH: &str = "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf";

fn new_pool(gfx: &mut Graphics, pools_size: u16) -> Vec<u16> {
    let mut pool = Vec::new();

    for i in 0..pools_size {
        pool.push(i+1);
    }

    let mut rng = rand::thread_rng();
    pool.shuffle(&mut rng);

    for index in 0..pools_size as usize {
        gfx.set_color(Color::new(0.0, 1.0, 0.5, 1.0));

        let x = (index as f32 * 2.0 / pools_size as f32) - 1.0;
        let width = 2.0 / pools_size as f32;
        gfx.fill_rect(
            x,
            -1.0,
            width,
            (pool[index] as f32 * ((2.0 - 0.2) / pool.len() as f32)),
        );
    }

    return pool;
}

fn sort(gfx: &mut Graphics, pool: &mut Vec<u16>) {
    gfx.clear(Color::new(0.0, 0.0, 0.0, 1.0));

    for mut index in 0..pool.len() as usize {
        if index < pool.len() -1 && pool[index] > pool[index + 1] {
            gfx.set_color(Color::new(0.0, 0.0, 0.8, 1.0));
            let moved = pool[index + 1];
            pool[index + 1] = pool[index];
            pool[index] = moved;
            
            
        } 
        let x = (index as f32 * 2.0 / pool.len() as f32) - 1.0;
        let width = 2.0 / pool.len() as f32;

        gfx.fill_rect(
            x,
            -1.0,
            width,
            (pool[index] as f32 * ((2.0 - 0.2) / pool.len() as f32)),
        );

        gfx.set_color(Color::new(0.0, 1.0, 0.5, 1.0));
    }
}

fn main() {
    let mut pool_size: u16 = 100;

    let mut win = Window::new(WIDTH, HEIGHT, "sorting").expect("could not create window!");
    win.make_current();
    init_gl(&mut win);

    let mut gfx = Graphics::new(&mut win);

    let mut gui = GUI::new(&mut win);


    let mut restart_btn = Button {
        x: -1.0,
        y: 1.0 - 0.2,
        width: 0.25,
        height: 0.2,
        text: String::from("sort"),
        pressed: false,
    };

    let mut amount_slider = Slider {
        x: -0.72,
        y: 1.0 - 0.1,
        width: 0.5,
        height: 0.01,
        val: 10.0 / pool_size as f32,
        selected: false,
    };

    let font = Font::new(FONT_PATH, 100);
    let gui_font = Font::new(FONT_PATH, 100);

    gui.style.text_align = TextAlign::Center;

    gui.graphics.set_font(gui_font);
    gfx.set_font(font);

    let mut pool = new_pool(&mut gfx, pool_size);

    let mut time_now = SystemTime::now();

    unsafe { engine_renderer::renderer::std_renderer::enable(engine_renderer::renderer::std_renderer::Capability::Blending); }
    unsafe { blend_func(BlendMode::SrcAlpha, BlendMode::OneMinusSrcAlpha);}

    while !win.should_close() {
        win.poll_events();
        win.swap_buffers();
      

        gfx.set_color(Color::new(0.0, 1.0, 0.5, 1.0));

        gui.button(&mut restart_btn);
        gui.slider(&mut amount_slider);


        gfx.draw_string(&format!("{}", (amount_slider.val * WIDTH as f32) as u32), -0.2, 1.0 - 0.15);

        if time_now.elapsed().unwrap() >= Duration::from_millis(100) {
        
            sort(&mut gfx, &mut pool);
            time_now = SystemTime::now();

        }
        
        if restart_btn.pressed {
            if amount_slider.val == 0.0 {
                pool = new_pool(&mut gfx, 1);
            } else {
                pool = new_pool(&mut gfx, (amount_slider.val * WIDTH as f32) as u16);
            }
        }
        gui.update();
        gfx.update();
        gfx.flush();
    
        
    }
}
