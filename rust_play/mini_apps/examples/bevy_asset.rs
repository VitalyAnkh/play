use bevy::prelude::*;
use image::{ImageBuffer, Rgba};
use std::path::Path;

fn save_mesh_system(meshes: Res<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    // Load the mesh asset
    let mesh_handle = asset_server.load("models/my_mesh.gltf#Mesh0/Primitive0");

    // Check if the mesh is loaded
    if let Some(mesh) = meshes.get(&mesh_handle) {
        // Access mesh attributes
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh has no positions");
        let indices = mesh.indices().expect("Mesh has no indices");

        // Convert mesh data to OBJ format (simplified example)
        let mut obj_data = String::new();

        // Write positions (vertices)
        if let VertexAttributeValues::Float32x3(positions) = positions {
            for position in positions {
                obj_data += &format!("v {} {} {}\n", position[0], position[1], position[2]);
            }
        }

        // Write faces (indices)
        match indices {
            Indices::U16(indices) => {
                for chunk in indices.chunks(3) {
                    obj_data += &format!("f {} {} {}\n", chunk[0] + 1, chunk[1] + 1, chunk[2] + 1);
                }
            }
            Indices::U32(indices) => {
                for chunk in indices.chunks(3) {
                    obj_data += &format!("f {} {} {}\n", chunk[0] + 1, chunk[1] + 1, chunk[2] + 1);
                }
            }
        }

        // Save the OBJ data to a file
        let mut file = File::create("assets/saved_mesh.obj").expect("Failed to create file");
        file.write_all(obj_data.as_bytes())
            .expect("Failed to write to file");

        println!("Mesh saved to assets/saved_mesh.obj");
    }
}

// System to save a texture
fn save_texture_system(images: Res<Assets<Image>>, asset_server: Res<AssetServer>) {
    // Load the texture asset
    let texture_handle = asset_server.load("textures/my_texture.png");

    // Check if the texture is loaded
    if let Some(image) = images.get(&texture_handle) {
        // Convert Bevy's Image data to ImageBuffer
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
            image.texture_descriptor.size.width,
            image.texture_descriptor.size.height,
            image.data.clone(),
        )
        .expect("Failed to create image buffer");

        // Save the image to disk
        buffer
            .save(Path::new("assets/saved_texture.png"))
            .expect("Failed to save texture");

        println!("Texture saved to assets/saved_texture.png");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (save_texture_system, save_mesh_system))
        .run();
}
