use bevy::prelude::*;
use crate::data::GameState;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      // .add_systems(OnEnter(GameState::PreStart), enter)
      .add_systems(Update, enter)
      // .add_systems(Update, update)
      ;
  }
}

fn enter(
  mut game_state_next: ResMut<NextState<GameState>>,
  mut local_res: ResMut<LocalResource>,
  time: Res<Time>,
) {
  // info!("enter()");

  if local_res.pre_start_timer.tick(time.delta()).just_finished() {
    game_state_next.set(GameState::Start);

    // info!("GameState::Start");
  }

  if local_res.play_timer.tick(time.delta()).just_finished() {
    game_state_next.set(GameState::Play);
    // info!("GameState::Play");
  }
}

fn update(
  mut _light_query: Query<&mut Transform, With<PointLight>>,
  _time: Res<Time>,
) {
  // let t = time.elapsed_seconds();
  // for mut tfm in light_query.iter_mut() {
  //   tfm.translation = 5.0 * Vec3::new(t.cos(), 1.0, t.sin());
  // }
}



#[derive(Resource)]
struct LocalResource {
  pre_start_timer: Timer,
  play_timer: Timer,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      pre_start_timer: Timer::from_seconds(3.0, TimerMode::Once),
      play_timer: Timer::from_seconds(6.0, TimerMode::Once),
    }
  }
}

