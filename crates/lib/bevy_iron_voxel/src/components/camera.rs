use bevy::prelude::*;
use bevy_flycam::FlyCam;
use bevy_voxel::BevyVoxelResource;
use crate::components::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, add);
  }
}

fn add(
  mut commands: Commands,
  query: Query<(Entity, &Player), Added<Player>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (entity, player) in &query {
    let rigid_body = &bevy_voxel_res.physics.rigid_body_set[player.body];
    let p = rigid_body.position().translation;
    let pos = Vec3::new(p.x, p.y, p.z);
    // let pos = Vec3::new(0.0, 15.0, 0.0);
    let forward = Vec3::new(0.69, -0.15, 0.70);


    info!("pos {}", pos);
    commands
      .entity(entity)
      .insert((
        Camera3dBundle {
          transform: Transform::from_translation(pos).looking_to(forward, Vec3::Y),
          ..Default::default()
        },
        FlyCam,
      ));
  }
}








