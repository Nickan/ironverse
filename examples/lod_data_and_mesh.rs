use std::f32::consts::PI;

use bevy::{prelude::*, window::{PresentMode, PrimaryWindow}, pbr::CascadeShadowConfigBuilder, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use bevy_egui::{EguiContexts, egui::{Frame, Color32, Pos2, Rect, Style, Vec2, RichText}, EguiPlugin};
use bevy_flycam::{NoCameraAndGrabPlugin, FlyCam, NoCameraPlayerPlugin};
use bevy_voxel::{Center, BevyVoxelResource};
use utils::Utils;
use voxels::{chunk::chunk_manager::ChunkManager, data::{voxel_octree::VoxelMode, surface_nets::VoxelReuse}};


fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Ironverse Editor".into(),
        resolution: (800., 600.).into(),
        present_mode: PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        ..default()
      }),
      ..default()
    }))
    .add_plugin(EguiPlugin)
    .add_plugin(NoCameraPlayerPlugin)
    .insert_resource(LocalResource::default())
    .insert_resource(BevyVoxelResource::default())
    .add_startup_system(startup)
    .add_startup_system(startup_lod)
    .add_system(show_diagnostic_texts)
    // .add_system(move_center)
    // .add_system(remove_out_of_range_meshes)
    // .add_system(add_mesh_by_delta_keys)
    .run();
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_translation(Vec3::new(12.7, 23.8, -18.9))
      .looking_to(Vec3::new(0.0, -0.6, 0.7), Vec3::Y),
      ..Default::default()
    })
    // .insert(Camera)
    .insert(FlyCam);

  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
      shadows_enabled: true,
      ..default()
    },
    transform: Transform {
      translation: Vec3::new(0.0, 2.0, 0.0),
      rotation: Quat::from_rotation_x(-PI / 4.),
      ..default()
    },
    // The default cascade config is designed to handle large scenes.
    // As this example has a much smaller world, we can tighten the shadow
    // bounds for better visual quality.
    cascade_shadow_config: CascadeShadowConfigBuilder {
      first_cascade_far_bound: 4.0,
      maximum_distance: 10.0,
      ..default()
    }
    .into(),
    ..default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(1.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    transform: Transform::from_xyz(0.0, 1.0, 0.0),
    ..default()
  })
  .insert(Center {
    prev_key: [0, 0, 0],
    key: [0, 0, 0],
  });
}

/// White for lod 0
/// Red for lod 1
/// Green for lod 2
/// Blue for lod 3
fn startup_lod(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut local_res: ResMut<LocalResource>,
  bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
/*   
  let ranges = local_res.ranges.clone();
  let key = [0, 0, 0];
  for lod in 0..ranges.len() - 1 {
    let keys = Utils::get_keys_by_lod(&ranges, &key, lod);
    for k in keys.iter() {
      if k[1] != -1 { continue; } // Ignore y

      let chunk = ChunkManager::new_chunk(
        k, 
        local_res.chunk_manager.depth as u8, 
        lod, 
        local_res.chunk_manager.noise,
      );
      let data = chunk.octree.compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut VoxelReuse::default(), 
        &local_res.colors, 
        1.0, 
        *k, 
        lod
      );

      let c = local_res.colors[lod];

      // let mut scale = data.lod as f32 * 2.0;
      // if scale == 0.0 {
      //   scale = 1.0;
      // }

      let scale = 1.0;

      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      let mesh_handle = meshes.add(render_mesh);

      let pos = bevy_voxel_res.get_pos(chunk.key);
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: materials.add(Color::rgb(c[0], c[1], c[2]).into()),
          transform: Transform::from_translation(pos)
            .with_scale(Vec3::new(scale, scale, scale)),
          ..default()
        })
      .insert(MeshGraphics { lod: lod });
    }
  } 
  */

  let depth = local_res.chunk_manager.depth as u8;
  let key = [0, -1, 0];
  let lod = 0;
  let chunk = ChunkManager::new_chunk(
    &key, 
    depth, 
    lod, 
    local_res.chunk_manager.noise,
  );
  let data = chunk.octree.compute_mesh(
    VoxelMode::SurfaceNets, 
    &mut VoxelReuse::new(depth as u32, 3), 
    &local_res.colors, 
    1.0, 
    key, 
    lod
  );

  let c = local_res.colors[lod];

  let mut scale = data.lod as f32 * 2.0;
  if scale == 0.0 {
    scale = 1.0;
  }

  // let scale = 1.0;

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  let mesh_handle = meshes.add(render_mesh);

  let pos = bevy_voxel_res.get_pos(chunk.key);
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: materials.add(Color::rgb(c[0], c[1], c[2]).into()),
      transform: Transform::from_translation(pos)
        .with_scale(Vec3::new(scale, scale, scale)),
      ..default()
    })
  .insert(MeshGraphics { lod: lod });


  let lod = 1;
  let depth = local_res.chunk_manager.depth as u8;
  let chunk = ChunkManager::new_chunk2(
    &key, 
    depth, 
    lod, 
    local_res.chunk_manager.noise,
  );
  let lod_depth = depth - lod as u8;
  let data = chunk.octree.compute_mesh(
    VoxelMode::SurfaceNets, 
    &mut VoxelReuse::new(lod_depth as u32, 3), 
    &local_res.colors, 
    1.0, 
    key, 
    lod
  );

  println!("data.indices.len() {}", data.indices.len());

  let c = local_res.colors[lod];

  let mut scale = data.lod as f32 * 2.0;
  if scale == 0.0 {
    scale = 1.0;
  }
  println!("scale {}", scale);

  // let scale = 1.0;

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  let mesh_handle = meshes.add(render_mesh);

  let pos = bevy_voxel_res.get_pos([1, -1, 0]);
  // let pos = bevy_voxel_res.get_pos(chunk.key);
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: materials.add(Color::rgb(c[0], c[1], c[2]).into()),
      transform: Transform::from_translation(pos)
        .with_scale(Vec3::new(scale, scale, scale)),
      ..default()
    })
  .insert(MeshGraphics { lod: lod });
}


fn move_center(
  mut centers: Query<(&mut Transform, &mut Center)>,
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {

  let mut vertical = 0.0;
  let mut horizontal = 0.0;
  let speed = 5.0;
  if key_input.pressed(KeyCode::W) {
    vertical = speed * time.delta_seconds();
  }
  if key_input.pressed(KeyCode::S) {
    vertical = -speed * time.delta_seconds();
  }
  if key_input.pressed(KeyCode::A) {
    horizontal = speed * time.delta_seconds();
  }
  if key_input.pressed(KeyCode::D) {
    horizontal = -speed * time.delta_seconds();
  }


  for (mut trans, mut center) in &mut centers {
    trans.translation.z += vertical;
    trans.translation.x += horizontal;

    let t = trans.translation;

    let key = [
      t.x as i64,
      0,
      t.z as i64
    ];
    if center.key != key {
      center.prev_key = center.key;
      center.key = key;
    }

  }
}


fn remove_out_of_range_meshes(
  mut commands: Commands,
  local_res: Res<LocalResource>,
  meshes: Query<(Entity, &Transform, &MeshGraphics)>,
  centers: Query<&Center, Changed<Center>>,
) {
  for center in &centers {
    for (entity, trans, mesh) in &meshes {
      let t = trans.translation;
      let mesh_key = [
        t.x as i64,
        0,
        t.z as i64
      ];

      if !Utils::in_range_by_lod(&center.key, &mesh_key, &local_res.ranges, mesh.lod) {
        commands.entity(entity).despawn_recursive();
      }
    }
  }
}

fn add_mesh_by_delta_keys(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,

  mut local_res: ResMut<LocalResource>,
  centers: Query<&Center, Changed<Center>>,
  mesh_graphics: Query<&MeshGraphics>,
) {
  /* How to prevent double adding the mesh at startup */
  for center in &centers {
    if center.prev_key == center.key {
      return;
    }
    let mut total_keys = 0;
    for lod in 0..local_res.ranges.len() - 1 {

    // for lod in 0..2 {
      let keys = Utils::get_delta_keys_by_lod(
        &local_res.ranges, &center.prev_key, &center.key, lod
      );

      // total_keys += keys.len();

      // println!(
      //   "prev_key {:?} key {:?} lod {} keys.len() {}", 
      //   center.prev_key, center.key, lod, keys.len()
      // );

      for k in keys.iter() {
        if k[1] != 0 { continue; } // Ignore y

        let c = local_res.colors[lod];
        commands.spawn(PbrBundle {
          mesh: meshes.add(shape::Plane::from_size(1.0).into()),
          material: materials.add(Color::rgb(c[0], c[1], c[2]).into()),
          transform: Transform::from_translation(
            Vec3::new(k[0] as f32, 0.0, k[2] as f32)
          ),
          ..default()
        })
        .insert(MeshGraphics { lod: lod });
      }
    }

    // println!("delta total_keys {}", total_keys);
  }

  if local_res.total_mesh != mesh_graphics.iter().len() {
    local_res.total_mesh = mesh_graphics.iter().len();
    println!(
      "total_key {} total_mesh {}", 
      local_res.total_keys, local_res.total_mesh
    );
  }
}


fn show_diagnostic_texts(
  cameras: Query<&Transform, With<FlyCam>>,
  mut windows: Query<&mut Window, With<PrimaryWindow>>,

  mut ctx: EguiContexts,
) {
  let mut window = match windows.get_single_mut() {
    Ok(w) => { w },
    Err(_e) => return,
  };

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  bevy_egui::egui::Window::new("ChunkTexts")
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

        for trans in &cameras {
          ui.label(
            RichText::new(format!("Pos: {:?}", trans.translation))
              .color(Color32::WHITE)
              .size(20.0)
          );

          ui.label(
            RichText::new(format!("Forward: {:?}", trans.forward()))
              .color(Color32::WHITE)
              .size(20.0)
          );
        }
      });
    });
}



#[derive(Resource)]
struct LocalResource {
  ranges: Vec<u32>,
  colors: Vec<[f32; 3]>,
  total_keys: usize,
  total_mesh: usize,
  chunk_manager: ChunkManager,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      ranges: vec![0, 1, 3, 5, 7],
      // ranges: vec![0, 1, 4, 8, 12],
      colors: vec![
        [1.0, 0.0, 0.0], 
        [0.0, 1.0, 0.0], 
        [0.0, 0.0, 1.0], 
        [0.0, 0.0, 0.0],

        [0.2, 0.0, 0.0],
        [0.4, 0.0, 0.0],
        [0.6, 0.0, 0.0],
        [0.8, 0.0, 0.0],

        [0.0, 0.2, 0.0],
        [0.0, 0.4, 0.0],
      ],
      total_keys: 0,
      total_mesh: 0,
      chunk_manager: ChunkManager::default(),
    }
  }
}


#[derive(Component)]
struct Camera;

#[derive(Component)]
struct MeshGraphics {
  lod: usize
}