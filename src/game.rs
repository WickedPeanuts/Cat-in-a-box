use raylib::{RaylibHandle, RaylibThread, color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::{entity2d::Entity2D, scene_texture_manager::SceneTextureManager};

pub struct Game {
    pub raylib_thread: RaylibThread,
    pub raylib_handle: RaylibHandle,

    pub window_size: Vector2,
    pub window_center: Vector2,

    pub scene_texture_manager: SceneTextureManager,
    pub entity_vect: Vec<Entity2D>
}

impl Game {
    pub fn new() -> Self {
        let (rl, thread) = raylib::init().build();
    
        let window_size = Vector2 { x : 800f32, y: 600f32 };
        let window_center = window_size / 2.0f32;

        return Self {
            raylib_thread: thread,
            raylib_handle: rl,

            window_size: window_size,
            window_center: window_center,

            scene_texture_manager: SceneTextureManager::new(),
            entity_vect: vec!()
        }
    }

    pub fn start_operation(&mut self) {
        self.raylib_handle.set_window_max_size(self.window_size.x as i32,self.window_size.y as i32);
        self.raylib_handle.set_window_min_size(self.window_size.x as i32, self.window_size.y as i32);
        self.raylib_handle.set_window_size(self.window_size.x as i32, self.window_size.y as i32);

        self.scene_texture_manager.load_all_textures(&self.raylib_thread, &mut self.raylib_handle);

        self.add_entity();

        while !self.raylib_handle.window_should_close() {
            self.update();
            self.draw();
        }

        self.unload();
    }

    fn add_entity(&mut self) {
        let mut entity_2d = Entity2D::new(
            "./assets/cat".to_owned(),
            Vector2::new(100.0f32, 100.0f32),
            Vector2::zero()
        );
        
        entity_2d.load(self);

        let source_rect_width = entity_2d.source_rect.width;
        let min_screen_ratio = f32::min(self.window_size.x, self.window_size.y);

        entity_2d.scale.x = min_screen_ratio / source_rect_width;
        entity_2d.scale.y = min_screen_ratio / source_rect_width;
        entity_2d.position = self.window_center;
        entity_2d.pivot = Vector2::one() * (min_screen_ratio / 2.0f32);

        self.entity_vect.push(entity_2d);
    }

    fn update(&mut self) {
        for entity_2d in self.entity_vect.iter_mut() {
            entity_2d.update(&mut self.raylib_handle);
        }
    }

    fn unload(&mut self) {
        // Making sure to release everyone that uses the Textures to have it unloaded automagically
        self.entity_vect.clear();
        self.scene_texture_manager.unload();
    }

    fn draw(&mut self) {
        let mut raylib_drawing_handle: RaylibDrawHandle = self.raylib_handle.begin_drawing(&self.raylib_thread);
        raylib_drawing_handle.clear_background(Color::BLACK);

        for entity_2d in self.entity_vect.iter_mut() {
            entity_2d.draw(&mut raylib_drawing_handle);
        }
    }
}