use std::rc::{Rc};

use raylib::{RaylibHandle, color::Color, core::math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibDrawHandle}, texture::Texture2D};
use crate::{game::Game};

pub struct Entity2D {
    pub position: Vector2,
    pub scale: Vector2,
    pub pivot: Vector2,

    pub source_rect: Rectangle,
    pub current_frame: usize,
    pub current_frame_time: f32,
    //pub is_animation_going_forward: bool,

    pub texture_2d_root_path: String,
    pub texture_2d_vector: Option<Rc<Vec<Texture2D>>>,
}

impl Entity2D {
    const TIME_PER_FRAME: f32 = 0.25f32;

    pub fn new(texture_2d_root_path: String, position: Vector2, pivot: Vector2) -> Self {
        Self {
            position: position,
            scale: Vector2::one(),
            pivot: pivot,
            source_rect: Rectangle::EMPTY,
            current_frame: 0,
            current_frame_time: 0.0,
            //is_animation_going_forward: true,
            texture_2d_root_path: texture_2d_root_path,
            texture_2d_vector: None
        }
    }

    fn get_current_frame(&self) -> &Texture2D {
        let unwrapped_texture_2d_vector = self.texture_2d_vector.as_ref().unwrap();
        let current_frame = &unwrapped_texture_2d_vector[self.current_frame];
        return current_frame;
    }

    pub fn update(&mut self, raylib_handle: &mut RaylibHandle) {
        self.update_animation_forward(raylib_handle);
    }

    fn update_animation_forward(&mut self, raylib_handle: &mut RaylibHandle) {
        self.current_frame_time += raylib_handle.get_frame_time();

        let clamped_value = f32::clamp(
            self.current_frame_time + raylib_handle.get_frame_time(),
            0.0f32, 1.0f32
        );

        if clamped_value < Self::TIME_PER_FRAME {
            return;
        }

        let unwrapped_texture_2d_vector = self.texture_2d_vector.as_ref().unwrap();
        let max_frame_num = unwrapped_texture_2d_vector.len() - 1;

        self.current_frame_time = 0.0;

        let next_frame = (self.current_frame + 1) & max_frame_num;
        self.current_frame = next_frame;
        
        println!("{}", self.current_frame);
    }

    /*
    fn update_animation_forward_and_back(&mut self, raylib_handle: &mut RaylibHandle) {
        self.current_frame_time += raylib_handle.get_frame_time();

        let clamped_value = f32::clamp(
            self.current_frame_time + raylib_handle.get_frame_time(),
            0.0f32, 1.0f32
        );

        if clamped_value < Self::TIME_PER_FRAME {
            return;
        }

        let unwrapped_texture_2d_vector = self.texture_2d_vector.as_ref().unwrap();
        let min_frame_num = 0usize;
        let max_frame_num = unwrapped_texture_2d_vector.len() - 1;

        self.current_frame_time = 0.0;

        if self.is_animation_going_forward {
            let next_frame = self.current_frame as i32 + 1;

            if next_frame > unwrapped_texture_2d_vector.len() as i32 - 1 {
                self.current_frame = usize::clamp((self.current_frame as i32 - 1) as usize, min_frame_num, max_frame_num);
                self.is_animation_going_forward = false;
            } else {
                self.current_frame = next_frame as usize;
            }
        } else {
            let next_frame = self.current_frame as i32 - 1;

            if next_frame < 0 {
                self.current_frame = usize::clamp(1, min_frame_num, max_frame_num);
                self.is_animation_going_forward = true;
            } else {
                self.current_frame = next_frame as usize;
            }
        }            
        println!("{}", self.current_frame);
    }
    */

    pub fn load(&mut self, game: &mut Game) {
        // Filling texture information
        game.scene_texture_manager.load_textures_by_directory(
            &game.raylib_thread,
            &mut game.raylib_handle,
            self.texture_2d_root_path.to_owned()
        );

        let text_vect_rc: Rc<Vec<Texture2D>> = game.scene_texture_manager.get(&self.texture_2d_root_path);

        // Storing Vector
        self.texture_2d_vector = Option::Some(text_vect_rc);
        
        let current_frame = self.get_current_frame();

        self.source_rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: current_frame.width as f32,
            height: current_frame.height as f32
        };
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle<'_>) {
        
        if self.texture_2d_vector.is_none() {
            return;
        }

        let current_frame = self.get_current_frame();

        let dest_rect = Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: ((current_frame.width as f32) * self.scale.x).floor(),
            height: ((current_frame.height as f32) * self.scale.y).floor()
        };

        
        draw_handle.draw_texture_pro(
            current_frame,
            self.source_rect,
            dest_rect,
            self.pivot,
            0.0, Color::WHITE);
    }
}