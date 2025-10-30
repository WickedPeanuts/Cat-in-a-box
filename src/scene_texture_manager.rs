use std::{collections::HashMap, fs, path::PathBuf, rc::{Rc}};

use raylib::{RaylibHandle, RaylibThread, texture::Texture2D};

pub struct SceneTextureManager {
    pub textures: HashMap<String, Rc<Vec<Texture2D>>>,
}

impl SceneTextureManager {
    const FAIL_TO_HANDLE_FILE: &str = "Failed to read config file";

    pub fn new() -> Self {
        Self {
            textures: HashMap::new()
        }
    }

    pub fn get(&self, texture_root_path: &str) -> Rc<Vec<Texture2D>> {
        let texture_2d_rc: &Rc<Vec<Texture2D>> = self.textures.get(texture_root_path).expect(Self::FAIL_TO_HANDLE_FILE);
        return texture_2d_rc.clone();
    }

    pub fn unload(&mut self) {
        self.textures.clear();
    }

    pub fn load_textures_by_directory(
        &mut self,
        raylib_thread: &RaylibThread,
        raylib_handle: &mut RaylibHandle,
        texture_root_path: String
    ) {  
        let mut files_contained: Vec<Texture2D> = vec!();
        
        let root_dir_entries = fs::read_dir(
            texture_root_path.to_owned()
        ).expect(Self::FAIL_TO_HANDLE_FILE);
        
        for dir_entry_result in root_dir_entries {
            let dir_entry = dir_entry_result.expect(Self::FAIL_TO_HANDLE_FILE);
            let dir_entry_path = dir_entry.path();

            if dir_entry_path.is_file() {
                let loaded_texture = raylib_handle.load_texture(
                    &raylib_thread,
                    dir_entry_path.to_str().expect(Self::FAIL_TO_HANDLE_FILE)
                ).expect(Self::FAIL_TO_HANDLE_FILE);
    
                files_contained.push(loaded_texture);
            }
        }

        self.textures.insert(texture_root_path.to_owned(), Rc::new(files_contained));
    }
    
    pub fn load_textures_by_directory_list(
        &mut self,
        raylib_thread: &RaylibThread,
        raylib_handle: &mut RaylibHandle,
        texture_root_path_list: &Vec<String>
    ) {
        texture_root_path_list.iter().for_each(
            | path: &String | self.load_textures_by_directory(
                raylib_thread,
                raylib_handle,
                path.to_owned()
            )
        );
    }

    pub fn load_all_textures(
        &mut self,
        raylib_thread: &RaylibThread,
        raylib_handle: &mut RaylibHandle,
    ) {
        let mut directories: Vec<PathBuf> = vec!();

        let root_entries = fs::read_dir("./assets").expect(Self::FAIL_TO_HANDLE_FILE);

        for entry in root_entries {
            let entry_path = entry.expect(Self::FAIL_TO_HANDLE_FILE).path();

            if !entry_path.is_file() {
                directories.push(entry_path);
            }
        }

        while directories.len() > 0 {
            let entry_path: PathBuf = directories.pop().expect(Self::FAIL_TO_HANDLE_FILE);

            let entries = fs::read_dir(entry_path).expect(Self::FAIL_TO_HANDLE_FILE);

            for entry in entries {
                let entry_path = entry.expect(Self::FAIL_TO_HANDLE_FILE).path();

                if !entry_path.is_file() {
                    directories.push(entry_path);
                }
            }
        }

        let found_directories_root_path: Vec<String> = directories.iter().map(
            |b| b.to_str().expect(Self::FAIL_TO_HANDLE_FILE).to_owned()
        ).collect();

        // Loading textures
        self.load_textures_by_directory_list(
            raylib_thread,
            raylib_handle,
            &found_directories_root_path
        );
    }
}

