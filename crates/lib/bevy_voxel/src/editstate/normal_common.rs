use bevy::prelude::*;
use rapier3d::prelude::ColliderHandle;
use utils::Utils;
use voxels::chunk::voxel_pos_to_key;
use crate::{EditState, Preview, BevyVoxelResource, Chunks, MeshComponent, Center};
use super::{EditEvents, EditEvent};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, preview_position.run_if(normal_state))
      .add_systems(Update, modify_voxels);
  }
}

fn normal_state(edit_state: Res<State<EditState>>,) -> bool {
  *State::get(&edit_state) == EditState::AddNormal
  // edit_state.0 == EditState::RemoveNormal
}

fn preview_position(
  mut cam: Query<(&Transform, &mut Preview), With<Preview>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);

    if hit.is_none() {
      if preview.pos.is_some() {
        preview.pos = None;
      }
      continue;
    }
    let point = hit.unwrap();
    let pos = bevy_voxel_res.get_nearest_voxel_air(point);
    if pos.is_none() && preview.pos.is_some() {
      preview.pos = pos;
    }

    if pos.is_some() {
      if preview.pos.is_some() {
        let p = pos.unwrap();
        let current = preview.pos.unwrap();
        if current != p {
          preview.pos = pos;
        }
      }
      
      if preview.pos.is_none() {
        preview.pos = pos;
      }
    }
  }
}


fn modify_voxels(
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &Preview, &mut Chunks, &mut MeshComponent)>,

  mut edit_event_reader: EventReader<EditEvents>,
) {
  for e in edit_event_reader.iter() {
    if e.event == EditEvent::AddCube {
      for (center, preview, mut chunks, mut mesh_comp) in &mut chunks {
        if preview.pos.is_none() {
          continue;
        }

        let p = preview.pos.unwrap();
        let res = bevy_voxel_res.set_voxel_cube(p, preview);

        let mut all_chunks = Vec::new();
        for (key, chunk) in res.iter() {
          all_chunks.push(chunk.clone());
          chunks.data.insert(*key, chunk.clone());
        }
        
        bevy_voxel_res.respawn_colliders(&center.key, &all_chunks, &mut mesh_comp);
      }
    }

    if e.event == EditEvent::AddSphere {
      for (center, preview, mut chunks, mut mesh_comp) in &mut chunks {
        if preview.pos.is_none() {
          continue;
        }

        let p = preview.pos.unwrap();
        let res = bevy_voxel_res.set_voxel_sphere(p, preview);

        let mut all_chunks = Vec::new();
        for (key, chunk) in res.iter() {
          all_chunks.push(chunk.clone());
          chunks.data.insert(*key, chunk.clone());
        }

        bevy_voxel_res.respawn_colliders(&center.key, &all_chunks, &mut mesh_comp);
      }
    }
  }
}

