use std::f64::consts::PI;
use std::time::Instant;

use crate::maths::vectors::V3;
use crate::rendering::{RenderConfig, Camera, self};
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct Interface 
{
    render_config: RenderConfig,
    camera: Camera
}

impl Interface {
    pub fn start(render_config: RenderConfig, camera: Camera) {
        let mut this = Self { render_config, camera };

        let (mut canvas, mut event_pump) = Self::initialise_display(&this);

        let mut now = Instant::now();

        'running: loop {
            let delta_time = now.elapsed().as_secs_f64();
            now = Instant::now();
    
            let exit = Self::handle_input(&mut this, &mut event_pump, delta_time);
            if exit { break 'running; }
            
            let mut pixel_data = this.camera.get_image(&this.render_config, false, false);
    
            let surface = Surface::from_data(&mut pixel_data, 
                this.render_config.resolution.0, 
                this.render_config.resolution.1, 
                3 * this.render_config.resolution.0, 
                sdl2::pixels::PixelFormatEnum::RGB24
            ).unwrap();
    
            canvas.copy(&surface.as_texture(&canvas.texture_creator()).unwrap(), None, None).unwrap();
            canvas.present();
        }
    }

    fn initialise_display(&self) -> (Canvas<Window>, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        sdl_context.mouse().show_cursor(false);
    
        let window = video_subsystem
            .window("Raytracing", self.render_config.resolution.0, self.render_config.resolution.1)
            .position_centered()
            .build()
            .unwrap();
    
        sdl_context.mouse().set_relative_mouse_mode(true);
    
        let canvas = window.into_canvas().build().unwrap();
    
        let event_pump = sdl_context.event_pump().unwrap();
    
        (canvas, event_pump)
    }

    fn handle_input(&mut self, event_pump: &mut EventPump, delta_time: f64) -> bool {
        for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::P), .. } => return true,
            Event::MouseMotion { xrel : x, yrel: y, .. } => 
                { self.camera.rotation.1 -= (y as f64) / 1000.0; 
                    self.camera.rotation.0 -= (x as f64) / 1000.0 }
            _ => {}
        }
        }

        let keyboard_state = event_pump.keyboard_state();

        let keys = keyboard_state.pressed_scancodes().filter_map(Keycode::from_scancode);

        for k in keys {
        match k {
            Keycode::W => { self.camera.position = &self.camera.position +
                &(&V3::FORWARD.rotate_y(&V3::ZERO, self.camera.rotation.0) * (delta_time * 20.0));
            },
            Keycode::A => { self.camera.position = &self.camera.position +
                &(&V3::LEFT.rotate_y(&V3::ZERO, self.camera.rotation.0) * (delta_time * 20.0));
            },
            Keycode::S => { self.camera.position = &self.camera.position +
                &(&V3::BACK.rotate_y(&V3::ZERO, self.camera.rotation.0) * (delta_time * 20.0));
            },
            Keycode::D => { self.camera.position = &self.camera.position +
                &(&V3::RIGHT.rotate_y(&V3::ZERO, self.camera.rotation.0) * (delta_time * 20.0));
            },
            Keycode::Q | Keycode::Space => { self.camera.position = &self.camera.position +
                &(&V3::UP * (delta_time * 20.0));
            },
            Keycode::E | Keycode::C | Keycode::LCtrl => { self.camera.position = &self.camera.position +
                &(&V3::DOWN * (delta_time * 20.0));
            },
            Keycode::Left => { self.camera.rotation.0 += 1.0 * delta_time; },
            Keycode::Right => { self.camera.rotation.0 -= 1.0 * delta_time; },
            Keycode::Down => { self.camera.rotation.1 -= 1.0 * delta_time; },
            Keycode::Up => { self.camera.rotation.1 += 1.0 * delta_time; },
            Keycode::M => { rendering::take_screenshot(&self.camera, &self.render_config); },
            _ => {}
        }
        }

        if self.camera.rotation.1 > (PI / 2.0) { self.camera.rotation.1 = PI / 2.0; }
        else if self.camera.rotation.1 < -(PI / 2.0) { self.camera.rotation.1 = -(PI / 2.0); }

        false
        }
}