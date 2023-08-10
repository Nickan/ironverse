use bevy::{prelude::*, render::{render_resource::{PrimitiveTopology, VertexFormat, AsBindGroup, RawRenderPipelineDescriptor, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}, mesh::{Indices, MeshVertexAttribute, MeshVertexBufferLayout}}, window::{PrimaryWindow, CursorGrabMode}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey}};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys}, utils::key_to_world_coord_f32, data::voxel_octree::{VoxelMode, VoxelOctree, MeshData}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(EguiPlugin)
      .add_startup_system(setup_camera)
      .add_startup_system(startup)
      .add_system(show_diagnostic_texts);
  }
}

fn setup_camera(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,

  mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
  commands
    .spawn(Camera3dBundle {
      // transform: Transform::from_xyz(6.5, 22.22, -8.4)
      //   .looking_to(Vec3::new(-0.0, -0.5, 0.8), Vec3::Y),
      transform: Transform::from_xyz(7.2, 37.40, 5.5)
        .looking_to(Vec3::new(-0.0, -0.99, 0.03), Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCam);

  // commands.spawn(PointLightBundle {
  //   point_light: PointLight {
  //     intensity: 6000.0,
  //     ..Default::default()
  //   },
  //   // transform: Transform::from_xyz(6.0, 30.0, 6.0),
  //   transform: Transform::from_xyz(6.0, 15.0, 6.0),
  //   ..Default::default()
  // });

  // Sun
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

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 6000.0,
      ..Default::default()
    },
    // transform: Transform::from_xyz(6.0, 30.0, 6.0),
    transform: Transform::from_xyz(6.0, 15.0, 6.0),
    ..Default::default()
  });

  let mut window = match windows.get_single_mut() {
    Ok(w) => { w },
    Err(_e) => return,
  };

  // window.cursor.grab_mode = CursorGrabMode::None;
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let size = 16;
  let seamless_size = 12;
  let scale = 0.5;

  let calc_size = (size - 2) as f32 * scale;

  // commands.spawn(PbrBundle {
  //   mesh: meshes.add(shape::Plane::from_size(calc_size).into()),
  //   transform: Transform::from_translation(
  //     Vec3::new(calc_size * 0.5, 3.0, calc_size * 0.5)
  //   ),
  //   material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.4).into()),
  //   ..default()
  // });


  let colors = vec![
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
  ];

  let manager = ChunkManager::default();
  let config = manager.config.clone();
  // let chunk = ChunkManager::new_chunk(&[0, -1, 0], config.depth, config.lod, config.noise);
  let mut chunk = Chunk::default();

  for x in 0..size {
    for y in 0..size {
      for z in 0..size {
        if y < 3 {
          chunk.octree.set_voxel(x, y, z, 1);
        }
      }
    }
  }


  let adj_keys = adjacent_keys(&[0, 0, 0], 1, true);

  for key in adj_keys.iter() {
    let chunk = ChunkManager::new_chunk(key, config.depth, config.lod, config.noise);

    let data = chunk
      .octree
      .compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut manager.voxel_reuse.clone(),
        &colors,
        scale
      );

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    let mesh_handle = meshes.add(render_mesh);

    let mut coord_f32 = key_to_world_coord_f32(key, manager.config.seamless_size);
    coord_f32[0] *= scale;
    coord_f32[1] *= scale;
    coord_f32[2] *= scale;
    commands
      .spawn(MaterialMeshBundle {
        mesh: mesh_handle,
        material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      });
  }


  
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

  if key_input.just_pressed(KeyCode::LControl) {
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


