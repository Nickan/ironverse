use bevy::{prelude::*, window::PrimaryWindow, diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore}};
use bevy_egui::{EguiContexts, egui::{self, Frame, Color32, Style, Rect, Vec2, Pos2, RichText}};
use bevy_voxel::{Preview, BevyVoxelResource};
use voxels::chunk::voxel_pos_to_key;
use crate::{components::{player::Player, chunk_edit::ChunkEdit}, graphics::ChunkGraphics};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(FrameTimeDiagnosticsPlugin::default())
      .add_systems(Update, show_texts)
      ;
  }
}

fn show_texts(
  mut ctx: EguiContexts,
  windows: Query<&Window, With<PrimaryWindow>>,
  diagnostics: Res<DiagnosticsStore>,

  time: Res<Time>,
  // mut local_res: ResMut<LocalResource>,

  players: Query<&Transform, With<Player>>,
  chunk_edits: Query<&ChunkEdit>,
  chunks: Query<&ChunkGraphics>,

  previews: Query<(&Transform, &Preview), With<Preview>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  let res = windows.get_single();
  if res.is_err() {
    return;
  }

  let fps = match diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
    Some(diag) => {
      let mut fps = 0.0;
      if diag.average().is_some() {
        fps = diag.average().unwrap()
      }
      fps
    },
    None => 0.0
  };

  // local_res.fps += 1.0;
  // // info!("test {}", local_res.fps);
  // if local_res.timer.tick(time.delta()).finished() {

  //   // info!("fps {}", local_res.fps);

  //   local_res.fps = 0.0;
  // }
  

  // info!("fps {:?}: {:?}", fps, settings.limiter);

  let window = res.unwrap();
  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  let mut player_pos = Vec3::ZERO;
  let mut forward = Vec3::ZERO;
  for trans in &players {
    player_pos = trans.translation.clone();
    forward = trans.forward();
  }

  let mut range_pos = Vec3::NAN;
  for edit in &chunk_edits {
    if edit.position.is_some() {
      range_pos = edit.position.unwrap();
    }
  }

  let total_meshes = chunks.iter().len();

  let mut preview_pos = None;
  let mut preview_key = None;
  for (trans, preview) in &previews {
    preview_pos = preview.pos;

    if preview_pos.is_some() {
      let p = preview.pos.unwrap();
      let pos = [
        p.x as i64,
        p.y as i64,
        p.z as i64,
      ];
      preview_key = Some(voxel_pos_to_key(&pos, 16));
    }
  }


  let mut total_colliders = bevy_voxel_res.physics.collider_set.len();

  egui::Window::new("DebuggerTexts")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect::from_min_max(
      Pos2::new(0.0, 0.0),
      Pos2::new(window.width(), window.height())
    ))
    .show(ctx.ctx_mut(), |ui| {
      ui.vertical(|ui| {
        let mut style = Style::default();
        style.spacing.item_spacing = Vec2::new(0.0, 0.0);
        ui.set_style(style);

        ui.add_space(400.0);

        ui.label(
          RichText::new(format!("FPS: {}", fps as u32))
            .color(Color32::BLACK)
            .size(20.0)
        );

        ui.label(
          RichText::new(format!("Pos: {:?}", player_pos))
            .color(Color32::BLACK)
            .size(20.0)
        );

        ui.label(
          RichText::new(format!("Preview pos: {:?}", preview_pos))
            .color(Color32::BLACK)
            .size(20.0)
        );

        ui.label(
          RichText::new(format!("Preview key: {:?}", preview_key))
            .color(Color32::BLACK)
            .size(20.0)
        );

        ui.label(
          RichText::new(format!("forward: {:?}", forward))
            .color(Color32::BLACK)
            .size(20.0)
        );

        ui.label(
          RichText::new(format!("Total Meshes: {}", total_meshes))
            .color(Color32::BLACK)
            .size(20.0)
        );

        ui.label(
          RichText::new(format!("Total colliders: {}", total_colliders))
            .color(Color32::WHITE)
            .size(20.0)
        );
        
      });
    });
}


/* #[derive(Resource)]
struct LocalResource {
  timer: Timer,
  fps: f32,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(1.0, TimerMode::Repeating),
      fps: 0.0,
    }
  }
} */