use bevy::prelude::*;
use bevy_voxel::BevyVoxelResource;
use crate::components::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, add)
      .add_systems(Update, follow_light);
  }
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  players: Query<(Entity, &Player), Added<Player>>,

  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (entity, player) in &players {
    let rigid_body = &bevy_voxel_res.physics.rigid_body_set[player.body];
    let p = rigid_body.position().translation;
    let pos = Vec3::new(p.x, p.y, p.z);

    commands
      .entity(entity)
      .insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(pos),
        ..default()
      });
  }
}

fn follow_light(
  mut light_query: Query<&mut Transform, With<PointLight>>,
  player: Query<&GlobalTransform, With<Player>>,
) {
  for mut tfm in light_query.iter_mut() {
    for global in &player {
      tfm.translation = global.translation();
    }
  }
}
