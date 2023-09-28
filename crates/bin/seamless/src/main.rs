use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PrimaryWindow, CursorGrabMode}};
use bevy_egui::{EguiContexts, egui::{Frame, Color32, Pos2, Rect, RichText, Style, Vec2}, EguiPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_voxel::{BevyVoxelPlugin, BevyVoxelResource};
use voxels::{data::{voxel_octree::{VoxelMode, VoxelOctree}, surface_nets::VoxelReuse}, chunk::chunk_manager::{self, ChunkManager}};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(NoCameraPlayerPlugin)
    .add_plugins(BevyVoxelPlugin)
    .add_plugin(EguiPlugin)
    .add_systems(Startup, setup)
    // .add_systems(Startup, old_mesh_system)
    // .add_systems(Startup, new_mesh_system)
    .add_systems(Startup, custom_octree_test)
    .add_systems(Update, show_diagnostic_texts)
    .run();
}

/// set up a simple 3D scene
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(1.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });

  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
      color: Color::rgb(0.98, 0.95, 0.82),
      shadows_enabled: true,
      illuminance: 10000.0,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 50.0, 0.0)
      .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ..default()
  });

  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(0.00, 4.36, -2.08)
        .looking_to(Vec3::new(0.24, -0.70, 0.66), Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCam);
}

fn old_mesh_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  let key = [0, 0, 0];
  let chunks = bevy_voxel_res.load_adj_chunks(key);
  for chunk in chunks.iter() {
    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
    if data.positions.len() == 0 {
      continue;
    }
    let pos = bevy_voxel_res.get_pos(chunk.key);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    let mut color = Color::rgb(0.7, 0.7, 0.7);
    if chunk.key[0] == key[0] && chunk.key[2] == key[0] {
      color = Color::rgb(1.0, 0.0, 0.0);
    }
    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render_mesh),
        material: materials.add(color.into()),
        transform: Transform::from_translation(pos),
        ..default()
      })
      // .insert(ChunkGraphics {
      //   handle: bevy_voxel_res.add_collider(pos, &data)
      // })
      ; 
  }
}

fn new_mesh_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  let key = [0, 0, 0];
  let chunks = bevy_voxel_res.load_adj_chunks(key);
  for chunk in chunks.iter() {
    let data = bevy_voxel_res.compute_mesh2(VoxelMode::SurfaceNets, chunk);
    if data.positions.len() == 0 {
      continue;
    }
    let pos = bevy_voxel_res.get_pos(chunk.key) + Vec3::new(-50.0, 0.0, 0.0);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    let mut color = Color::rgb(0.7, 0.7, 0.7);
    if chunk.key[0] == key[0] && chunk.key[2] == key[0] {
      color = Color::rgb(1.0, 0.0, 0.0);
    }
    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render_mesh),
        material: materials.add(color.into()),
        transform: Transform::from_translation(pos),
        ..default()
      })
      // .insert(ChunkGraphics {
      //   handle: bevy_voxel_res.add_collider(pos, &data)
      // })
      ; 
  }
}


fn custom_octree_test(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  let depth = 3;

  let mut octree = VoxelOctree::new(0, depth);
  octree.set_voxel(1, 1, 1, 1);


  let chunk_manager = ChunkManager::new_1(depth.into());

  let data = octree.compute_mesh2(
    VoxelMode::SurfaceNets, 
    &chunk_manager, 
    [0, 0, 0], 
    0,
  );

  // let data = octree.compute_mesh(
  //   VoxelMode::SurfaceNets, 
  //   &mut VoxelReuse::new(depth.into(), 3),
  //   &&chunk_manager.colors,
  //   chunk_manager.voxel_scale, 
  //   [0, 0, 0], 
  //   0,
  // );

  println!("data.indices {}", data.indices.len());

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  commands
    .spawn(MaterialMeshBundle {
      mesh: meshes.add(render_mesh),
      material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
      transform: Transform::from_translation(Vec3::ZERO),
      ..default()
    }); 
}



fn show_diagnostic_texts(
  cameras: Query<&Transform, With<FlyCam>>,
  mut windows: Query<&mut Window, With<PrimaryWindow>>,
  key_input: Res<Input<KeyCode>>,

  mut ctx: EguiContexts,
) {
  let mut window = match windows.get_single_mut() {
    Ok(w) => { w },
    Err(_e) => return,
  };

  if key_input.just_pressed(KeyCode::ControlLeft) {
    match window.cursor.grab_mode {
      CursorGrabMode::None => {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
      }
      _ => {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
      }
    }
  }
  

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



// #[derive(Component, Clone)]
// struct ChunkGraphics {
//   pub handle: ColliderHandle,
// }
