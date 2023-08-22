use bevy::prelude::*;
use bevy_voxel::BevyVoxelResource;
use rapier3d::{na::Vector3, prelude::{RigidBodyHandle, ColliderHandle}};
use voxels::utils::posf32_to_world_key;
use crate::{physics::Physics, data::{GameResource, GameState}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(start.in_schedule(OnEnter(GameState::Start)))
      .add_system(init.in_schedule(OnEnter(GameState::Init)))
      .add_system(update);
  }
}

fn start(
  mut commands: Commands,
  mut physics: ResMut<Physics>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  // // let pos = [0.0, 5.0, 0.0];
  let pos = Vec3::new(0.0, 0.4, 0.0);

  let (body, collider) = physics.spawn_character(1.0, 0.5, pos);
  let k = bevy_voxel_res.get_key(pos);
  commands
    .spawn(
      (Player::new(body, collider, k),
    ));
}

fn init(
  mut commands: Commands,
  mut physics: ResMut<Physics>,
  bevy_voxel_res: Res<BevyVoxelResource>,
  game_res: Res<GameResource>,
) {
  let p = game_res.data.status.position;
  let pos = Vec3::new(p[0], p[1], p[2]);
  let (body, collider) = physics.spawn_character(
    1.0, 0.5, pos
  );
  let k = bevy_voxel_res.get_key(pos);
  commands
    .spawn(
      (Player::new(body, collider, k),
    ));

  // info!("player init() {:?}", pos);
}

fn update(
  mut query: Query<(&Transform, &mut Player)>,
  mut physics: ResMut<Physics>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (trans, mut player) in &mut query {
    let p = trans.translation;
    let rigid_body = &mut physics.rigid_body_set[player.body];
    rigid_body.set_position(Vector3::new(p.x, p.y, p.z).into(), false);

    let k = bevy_voxel_res.get_key(p);
    if player.key != k {
      println!("player.key {:?}", player.key);

      player.prev_key = player.key.clone();
      player.key = k;
    }
  }
}

#[derive(Component, Debug, Clone)]
pub struct Player {
  pub body: RigidBodyHandle,
  pub collider: ColliderHandle,
  pub prev_key: [i64; 3],
  pub key: [i64; 3],
}

impl Player {
  pub fn new(b: RigidBodyHandle, c: ColliderHandle, k: [i64; 3]) -> Self {
    
    Self {
      body: b,
      collider: c,
      prev_key: k.clone(),
      key: k
    }
  }
}