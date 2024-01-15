use bevy::prelude::*;
use crate::data::GameState;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      // .add_system(
      //   enter.in_schedule(OnEnter(GameState::Init))
      // )
      .add_system(
        enter
      )
      // .add_system(update)
      ;
  }
}

fn enter(
  mut game_state_next: ResMut<NextState<GameState>>,
  time: Res<Time>,
  mut local_res: ResMut<LocalResource>,
) {

  if local_res.timer.finished() {
    // game_state_next.set(GameState::Play);
  }
  
}

fn update(
  mut light_query: Query<&mut Transform, With<PointLight>>,
  time: Res<Time>,
) {
  let t = time.elapsed_seconds();
  for mut tfm in light_query.iter_mut() {
    tfm.translation = 5.0 * Vec3::new(t.cos(), 1.0, t.sin());
  }
}



#[derive(Resource)]
struct LocalResource {
  timer: Timer,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(3.0, TimerMode::Once)
    }
  }
}

