use bevy::prelude::*;
use rapier3d::{prelude::{Vector, ColliderHandle, Ray, QueryFilter}, na::Point3};
use utils::RayUtils;
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys}, data::{voxel_octree::{VoxelMode, MeshData}, surface_nets::VoxelReuse}};
use voxels::utils::key_to_world_coord_f32;
use crate::{BevyVoxelResource, physics::Physics};
use crate::util::*;

impl BevyVoxelResource {

  pub fn new(
    depth: u32, 
    voxel_scale: f32, 
    range: u8,
    colors: Vec<[f32; 3]>,  
  ) -> Self {
    Self {
      chunk_manager: ChunkManager::new(
        depth,
        voxel_scale,
        range,
        colors,
      ),
      physics: Physics::default(),
      colliders_cache: Vec::new(),
    }
  }

  pub fn get_key(&self, pos: Vec3) -> [i64; 3] {
    let scale = self.chunk_manager.voxel_scale;
    let seamless_size = self.chunk_manager.seamless_size();
    get_key(pos, scale, seamless_size)
  }

  /// Get all chunks adjacent to the player based on
  /// Depth, range and voxel scale
  pub fn load_adj_chunks(&mut self, key: [i64; 3]) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    let keys = adjacent_keys(&key, self.chunk_manager.range as i64, true);
    for key in keys.iter() {
      chunks.push(load_chunk(self, *key));
      
    }

    chunks
  }

  /// Return mesh data needed for collision or rendering
  pub fn compute_mesh(&self, mode: VoxelMode, chunk: &Chunk) -> MeshData {
    chunk
      .octree
      .compute_mesh(
        mode, 
        &mut VoxelReuse::new(self.chunk_manager.depth, 3),
        &self.chunk_manager.colors,
        self.chunk_manager.voxel_scale
      )
  }

  /// Return a world position based on chunk size(depth) and voxel scale
  pub fn get_pos(&self, key: [i64; 3]) -> Vec3 {
    let seamless = self.chunk_manager.seamless_size();
    let scale = self.chunk_manager.voxel_scale;
    let mut pos = key_to_world_coord_f32(&key, seamless);

    pos[0] *= scale;
    pos[1] *= scale;
    pos[2] *= scale;
    
    Vec3::new(pos[0], pos[1], pos[2])
  }


  pub fn get_raycast_hit(&self, trans: &Transform) -> Option<Vec3> {
    let start_pos = trans.translation;
    let dir = trans.forward();
    let ray = Ray::new(
      Point3::new(start_pos.x, start_pos.y, start_pos.z), 
      Vector::new(dir.x, dir.y, dir.z)
    );
    let max_toi = f32::MAX;
    let solid = true;
    let filter = QueryFilter::only_fixed();

    let mut point = None;
    if let Some((_handle, toi)) = self.physics.query_pipeline.cast_ray(
      &self.physics.rigid_body_set, 
      &self.physics.collider_set, 
      &ray, 
      max_toi, 
      solid, 
      filter
    ) {
      let hit = ray.point_at(toi);
      point = Some(Vec3::new(hit[0], hit[1], hit[2]));
    }
    point
  }

  pub fn get_hit_voxel_pos(&self, point: Vec3) -> Option<Vec3> {
    let voxel_scale = self.chunk_manager.voxel_scale;
    let mul = 1.0 / voxel_scale;
    let mut nearest_dist = f32::MAX;

    let mut pos = None;
    let n_c = RayUtils::get_nearest_coord(
      [point.x, point.y, point.z], voxel_scale
    );

    let tmp_point = Vec3::new(n_c[0], n_c[1], n_c[2]);
    let near_pos = get_near_positions(tmp_point, voxel_scale);
    for n in near_pos.iter() {
      let dist = point.distance(*n);
      let tmp_pos = [
        (n[0] * mul) as i64, 
        (n[1] * mul) as i64, 
        (n[2] * mul) as i64, 
      ];

      let res = self.chunk_manager.get_voxel_safe(&tmp_pos);
      if res.is_some() && res.unwrap() != 0 {
        if dist < nearest_dist {
          nearest_dist = dist;
          pos = Some(*n);
        }
      }
    }

    pos
  }

  pub fn get_nearest_voxel_air(&self, point: Vec3) -> Option<Vec3> {
    let voxel_scale = self.chunk_manager.voxel_scale;

    let mul = 1.0 / voxel_scale;
    let mut nearest_dist = f32::MAX;

    let mut pos = None;
    let n_c = RayUtils::get_nearest_coord(
      [point.x, point.y, point.z], voxel_scale
    );

    let tmp_point = Vec3::new(n_c[0], n_c[1], n_c[2]);
    let near_pos = get_near_positions(tmp_point, voxel_scale);
    
    for n in near_pos.iter() {
      let dist = point.distance(*n);
      let tmp_pos = [
        (n[0] * mul) as i64, 
        (n[1] * mul) as i64, 
        (n[2] * mul) as i64, 
      ];

      let res = self.chunk_manager.get_voxel_safe(&tmp_pos);
      if res.is_some() && res.unwrap() == 0 {
        if dist < nearest_dist {
          nearest_dist = dist;
          pos = Some(*n);
        }
      }
    }

    // println!("point {:?} : {:?}", point, pos);
    pos
  }

  /// - calc_pos should be the calculated position based on edit mode
  /// - Add voxel mode(TODO): Probably be a separate function
  /// - Remove voxel mode(TODO): Probably be a separate function
  pub fn get_preview_chunk(&self, calc_pos: Vec3, voxel: u8) -> Chunk {
    let mul = (1.0 / self.chunk_manager.voxel_scale);
    let p = [
      calc_pos.x * mul,
      calc_pos.y * mul,
      calc_pos.z * mul,
    ];

    let mut tmp_manager = self.chunk_manager.clone();
    set_voxel(&mut tmp_manager, calc_pos, voxel);

    let mut chunk = Chunk::default();
    let mid_pos = (chunk.octree.get_size() / 2) as i64;

    let preview_size = 3;
    let min = -preview_size;
    let max = preview_size;
    for x in min..max {
      for y in min..max {
        for z in min..max {
          let local_x = (mid_pos + x) as u32;
          let local_y = (mid_pos + y) as u32;
          let local_z = (mid_pos + z) as u32;

          let tmp_pos = [
            p[0] as i64 + x,
            p[1] as i64 + y,
            p[2] as i64 + z,
          ];
          let v = tmp_manager.get_voxel(&tmp_pos);
          chunk.octree.set_voxel(local_x, local_y, local_z, v);
        }
      }
    }
    
    chunk
  }

  /// Get preview chunk pos converted to world pos considering the size of chunk
  /// and positioned visually correct
  pub fn get_preview_pos(&self, calc_pos: Vec3) -> Vec3 {
    let scale = self.chunk_manager.voxel_scale;
    let mid_pos = (self.chunk_manager.chunk_size / 2) as f32 * scale;
    Vec3::new(
      calc_pos.x - mid_pos,
      calc_pos.y - mid_pos,
      calc_pos.z - mid_pos,
    )
  }


  pub fn set_voxel(&mut self, pos: Vec3, voxel: u8) {
    let mul = 1.0 / self.chunk_manager.voxel_scale;
    let p = [
      (pos.x * mul) as i64,
      (pos.y * mul) as i64,
      (pos.z * mul) as i64,
    ];

    self.chunk_manager.set_voxel2(&p, voxel);
  }


  pub fn load_adj_chunks_with_collider(&mut self, key: [i64; 3]) -> Vec<Chunk> {
    let chunks = self.load_adj_chunks(key);

    for _ in 0..self.colliders_cache.len() {
      let h = self.colliders_cache.pop().unwrap();
      self.remove_collider(h);
    }    

    self.colliders_cache.clear();

    for chunk in chunks.iter() {
      let data = self.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }

      let pos = self.get_pos(chunk.key);
      let c = self.add_collider(pos, &data);
      self.colliders_cache.push(c);
    }

    chunks
  }


  pub fn add_collider(
    &mut self, 
    pos: Vec3, 
    data: &MeshData
  ) -> ColliderHandle {
    self.physics.add_collider(
      [pos.x, pos.y, pos.z], &data.positions, &data.indices
    )
  }

  pub fn remove_collider(&mut self, handle: ColliderHandle) {
    self.physics.remove_collider(handle);
  }


}