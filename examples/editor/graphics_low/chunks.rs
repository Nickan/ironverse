use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use bevy_voxel::{BevyVoxelResource, Chunks, MeshComponent, Center};
use utils::Utils;
use crate::graphics::ChunkGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add)
      .add_system(remove);
  }
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &MeshComponent), Changed<MeshComponent>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {

  for (_, mesh_comp) in &chunk_query {
    for data in mesh_comp.added.iter() {
      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      let mesh_handle = meshes.add(render_mesh);
      let mut pos = bevy_voxel_res.get_pos(data.key);

      let mat = materials.add(Color::rgb(0.7, 0.7, 0.7).into());
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: mat,
          transform: Transform::from_translation(pos),
          ..default()
        })
        .insert(ChunkGraphics { key: data.key, lod: 4 });
    }
  }
}

fn remove(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Center), Changed<MeshComponent>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {

  for (_, center) in &chunk_query {
    for (entity, graphics) in &chunk_graphics {

      if graphics.lod == 4 && 
      Utils::get_tile_range(&center.key, &graphics.key) > 1 {
        commands.entity(entity).despawn_recursive();
      }
      
    }
  }
}