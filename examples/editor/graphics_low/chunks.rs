use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use bevy_voxel::{BevyVoxelResource, Chunks, MeshComponent, Center};
use utils::Utils;
use crate::graphics::ChunkGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(add)
      .add_system(remove);
  }
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  mut chunk_query: Query<(Entity, &mut MeshComponent), Changed<MeshComponent>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {

  for (_, mut mesh_comp) in &mut chunk_query {
    for (data, collider_handle) in mesh_comp.added.iter() {
      for (entity, graphics) in &chunk_graphics {
        if graphics.key == data.key {
          commands.entity(entity).despawn();
        }
      }

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
        .insert(ChunkGraphics { 
          key: data.key, 
          lod: data.lod as usize,
          collider: *collider_handle,
        });

      // println!("data.lod {}", data.lod);
    }
    mesh_comp.added.clear();
  }
}

fn remove(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Center)>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {

  let ranges = bevy_voxel_res.ranges.clone();
  for (_, center) in &chunk_query {
    for (entity, graphics) in &chunk_graphics {

      if !bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, graphics.lod) {
        commands.entity(entity).despawn_recursive();
        bevy_voxel_res.physics.remove_collider(graphics.collider);
      }
      
    }
  }
}

#[derive(Resource)]
struct LocalResource {
  total_keys: usize,  // For testing
  total_mesh: usize,  // For testing
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      total_keys: 0,
      total_mesh: 0,
    }
  }
}