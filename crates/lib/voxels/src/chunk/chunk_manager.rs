use crate::{data::{voxel_octree::{VoxelOctree, ParentValueType}, surface_nets::VoxelReuse}, utils::get_chunk_coords};
use super::*;
use hashbrown::HashMap;
use noise::*;
use serde::{Serialize, Deserialize};

pub const DEFAULT_COLOR_PALETTE: [[f32; 3]; 255] = [
  [1.00, 1.00, 1.00], [0.80, 0.93, 0.80], [0.93, 0.93, 0.67], [0.80, 0.93, 0.53], [0.93, 0.93, 0.40], [0.80, 0.93, 0.27], [0.93, 0.93, 0.13], [0.80, 0.93, 0.00],
  [0.67, 0.93, 0.93], [0.53, 0.93, 0.80], [0.67, 0.93, 0.67], [0.53, 0.93, 0.53], [0.67, 0.93, 0.40], [0.53, 0.93, 0.27], [0.67, 0.93, 0.13], [0.53, 0.93, 0.00],
  [0.40, 0.93, 0.93], [0.27, 0.93, 0.80], [0.40, 0.93, 0.67], [0.27, 0.93, 0.53], [0.40, 0.93, 0.40], [0.27, 0.93, 0.27], [0.40, 0.93, 0.13], [0.27, 0.93, 0.00],
  [0.13, 0.93, 0.93], [0.00, 0.93, 0.80], [0.13, 0.93, 0.67], [0.00, 0.93, 0.53], [0.13, 0.93, 0.40], [0.00, 0.93, 0.27], [0.13, 0.93, 0.13], [0.00, 0.93, 0.00],
  [0.93, 0.80, 0.80], [0.80, 0.80, 0.80], [0.93, 0.80, 0.67], [0.80, 0.80, 0.53], [0.93, 0.80, 0.40], [0.80, 0.80, 0.27], [0.93, 0.80, 0.13], [0.80, 0.80, 0.00],
  [0.67, 0.80, 0.93], [0.53, 0.80, 0.80], [0.67, 0.80, 0.67], [0.53, 0.80, 0.53], [0.67, 0.80, 0.40], [0.53, 0.80, 0.27], [0.67, 0.80, 0.13], [0.53, 0.80, 0.00],
  [0.40, 0.80, 0.93], [0.27, 0.80, 0.80], [0.40, 0.80, 0.67], [0.27, 0.80, 0.53], [0.40, 0.80, 0.40], [0.27, 0.80, 0.27], [0.40, 0.80, 0.13], [0.27, 0.80, 0.00],
  [0.13, 0.80, 0.93], [0.00, 0.80, 0.80], [0.13, 0.80, 0.67], [0.00, 0.80, 0.53], [0.13, 0.80, 0.40], [0.00, 0.80, 0.27], [0.13, 0.80, 0.13], [0.00, 0.80, 0.00], 
  [0.93, 0.67, 0.93], [0.80, 0.67, 0.80], [0.93, 0.67, 0.67], [0.80, 0.67, 0.53], [0.93, 0.67, 0.40], [0.80, 0.67, 0.27], [0.93, 0.67, 0.13], [0.80, 0.67, 0.00],
  [0.67, 0.67, 0.93], [0.53, 0.67, 0.80], [0.67, 0.67, 0.67], [0.53, 0.67, 0.53], [0.67, 0.67, 0.40], [0.53, 0.67, 0.27], [0.67, 0.67, 0.13], [0.53, 0.67, 0.00],
  [0.40, 0.67, 0.93], [0.27, 0.67, 0.80], [0.40, 0.67, 0.67], [0.27, 0.67, 0.53], [0.40, 0.67, 0.40], [0.27, 0.67, 0.27], [0.40, 0.67, 0.13], [0.27, 0.67, 0.00],
  [0.13, 0.67, 0.93], [0.00, 0.67, 0.80], [0.13, 0.67, 0.67], [0.00, 0.67, 0.53], [0.13, 0.67, 0.40], [0.00, 0.67, 0.27], [0.13, 0.67, 0.13], [0.00, 0.67, 0.00],
  [0.93, 0.53, 0.93], [0.80, 0.53, 0.80], [0.93, 0.53, 0.67], [0.80, 0.53, 0.53], [0.93, 0.53, 0.40], [0.80, 0.53, 0.27], [0.93, 0.53, 0.13], [0.80, 0.53, 0.00],
  [0.67, 0.53, 0.93], [0.53, 0.53, 0.80], [0.67, 0.53, 0.67], [0.53, 0.53, 0.53], [0.67, 0.53, 0.40], [0.53, 0.53, 0.27], [0.67, 0.53, 0.13], [0.53, 0.53, 0.00],
  [0.40, 0.53, 0.93], [0.27, 0.53, 0.80], [0.40, 0.53, 0.67], [0.27, 0.53, 0.53], [0.40, 0.53, 0.40], [0.27, 0.53, 0.27], [0.40, 0.53, 0.13], [0.27, 0.53, 0.00],
  [0.13, 0.53, 0.93], [0.00, 0.53, 0.80], [0.13, 0.53, 0.67], [0.00, 0.53, 0.53], [0.13, 0.53, 0.40], [0.00, 0.53, 0.27], [0.13, 0.53, 0.13], [0.00, 0.53, 0.00],
  [0.93, 0.40, 0.93], [0.80, 0.40, 0.80], [0.93, 0.40, 0.67], [0.80, 0.40, 0.53], [0.93, 0.40, 0.40], [0.80, 0.40, 0.27], [0.93, 0.40, 0.13], [0.80, 0.40, 0.00],
  [0.67, 0.40, 0.93], [0.53, 0.40, 0.80], [0.67, 0.40, 0.67], [0.53, 0.40, 0.53], [0.67, 0.40, 0.40], [0.53, 0.40, 0.27], [0.67, 0.40, 0.13], [0.53, 0.40, 0.00],
  [0.40, 0.40, 0.93], [0.27, 0.40, 0.80], [0.40, 0.40, 0.67], [0.27, 0.40, 0.53], [0.40, 0.40, 0.40], [0.27, 0.40, 0.27], [0.40, 0.40, 0.13], [0.27, 0.40, 0.00],
  [0.13, 0.40, 0.93], [0.00, 0.40, 0.80], [0.13, 0.40, 0.67], [0.00, 0.40, 0.53], [0.13, 0.40, 0.40], [0.00, 0.40, 0.27], [0.13, 0.40, 0.13], [0.00, 0.40, 0.00],
  [0.93, 0.27, 0.93], [0.80, 0.27, 0.80], [0.93, 0.27, 0.67], [0.80, 0.27, 0.53], [0.93, 0.27, 0.40], [0.80, 0.27, 0.27], [0.93, 0.27, 0.13], [0.80, 0.27, 0.00],
  [0.67, 0.27, 0.93], [0.53, 0.27, 0.80], [0.67, 0.27, 0.67], [0.53, 0.27, 0.53], [0.67, 0.27, 0.40], [0.53, 0.27, 0.27], [0.67, 0.27, 0.13], [0.53, 0.27, 0.00],
  [0.40, 0.27, 0.93], [0.27, 0.27, 0.80], [0.40, 0.27, 0.67], [0.27, 0.27, 0.53], [0.40, 0.27, 0.40], [0.27, 0.27, 0.27], [0.40, 0.27, 0.13], [0.27, 0.27, 0.00],
  [0.13, 0.27, 0.93], [0.00, 0.27, 0.80], [0.13, 0.27, 0.67], [0.00, 0.27, 0.53], [0.13, 0.27, 0.40], [0.00, 0.27, 0.27], [0.13, 0.27, 0.13], [0.00, 0.27, 0.00],
  [0.93, 0.13, 0.93], [0.80, 0.13, 0.80], [0.93, 0.13, 0.67], [0.80, 0.13, 0.53], [0.93, 0.13, 0.40], [0.80, 0.13, 0.27], [0.93, 0.13, 0.13], [0.80, 0.13, 0.00],
  [0.67, 0.13, 0.93], [0.53, 0.13, 0.80], [0.67, 0.13, 0.67], [0.53, 0.13, 0.53], [0.67, 0.13, 0.40], [0.53, 0.13, 0.27], [0.67, 0.13, 0.13], [0.53, 0.13, 0.00],
  [0.40, 0.13, 0.93], [0.27, 0.13, 0.80], [0.40, 0.13, 0.67], [0.27, 0.13, 0.53], [0.40, 0.13, 0.40], [0.27, 0.13, 0.27], [0.40, 0.13, 0.13], [0.27, 0.13, 0.00],
  [0.13, 0.13, 0.93], [0.00, 0.13, 0.80], [0.13, 0.13, 0.67], [0.00, 0.13, 0.53], [0.13, 0.13, 0.40], [0.00, 0.13, 0.27], [0.13, 0.13, 0.13], [0.00, 0.13, 0.00],
  [0.93, 0.00, 0.93], [0.80, 0.00, 0.80], [0.93, 0.00, 0.67], [0.80, 0.00, 0.53], [0.93, 0.00, 0.40], [0.80, 0.00, 0.27], [0.93, 0.00, 0.13], [0.80, 0.00, 0.00], 
  [0.67, 0.00, 0.93], [0.53, 0.00, 0.80], [0.67, 0.00, 0.67], [0.53, 0.00, 0.53], [0.67, 0.00, 0.40], [0.53, 0.00, 0.27], [0.67, 0.00, 0.13], [0.53, 0.00, 0.00],
  [0.40, 0.00, 0.93], [0.27, 0.00, 0.80], [0.40, 0.00, 0.67], [0.27, 0.00, 0.53], [0.40, 0.00, 0.40], [0.27, 0.00, 0.27], [0.40, 0.00, 0.13], [0.27, 0.00, 0.00],
  [0.13, 0.00, 0.93], [0.00, 0.00, 0.80], [0.13, 0.00, 0.67], [0.00, 0.00, 0.53], [0.13, 0.00, 0.40], [0.00, 0.00, 0.27], [0.13, 0.00, 0.13]
];

#[derive(Default)]
pub struct LoadedChunk {
  pub key: [u32; 3],
  pub ttl: f32,
}

#[derive(Default)]
pub struct SubscribeData {
  pub chunks: HashMap<[u32; 3], VoxelOctree>,
  pub rays: Vec<[f32; 3]>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ChunkMode {
  None,
  Loaded,
  Unloaded,
  Air,
  Inner,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deployment {
  Production,
  Development,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Chunk {
  pub key: [i64; 3],
  pub lod: usize,
  pub octree: VoxelOctree,
  // pub neighbor_data: [],
  pub mode: ChunkMode,
  pub is_default: bool,
}

impl Default for Chunk {
  fn default() -> Chunk {
    Chunk {
      key: [0, 0, 0],
      lod: 4,
      octree: VoxelOctree::new(0, 4),
      mode: ChunkMode::Unloaded,
      is_default: true,
    }
  }
}

impl Chunk {
  fn new(key: [i64; 3], depth: u8) -> Self {
    Self {
      key: key,
      lod: 4,
      octree: VoxelOctree::new(0, depth),
      mode: ChunkMode::Unloaded,
      is_default: true,
    }
  }
}


#[derive(Clone)]
pub struct ChunkManager {
  pub chunks: HashMap<[i64; 3], Chunk>,
  pub depth: u32,
  pub chunk_size: u32,
  pub offset: u32,
  pub noise: OpenSimplex,
  pub height_scale: f64,
  pub frequency: f64,

  pub voxel_scale: f32,
  pub range: u8,
  pub colors: Vec<[f32; 3]>,

  pub voxel_reuse: VoxelReuse,
}

impl Default for ChunkManager {
  fn default() -> Self {
    let depth = 4;
    let loop_count = 3; // indices/axes being used, [x, y, z]
    let voxel_reuse = VoxelReuse::new(depth, loop_count);
    
    let noise = OpenSimplex::new().set_seed(1234);
    let offset = 2;
    let chunk_size = 2_i32.pow(depth) as u32;

    ChunkManager {
      chunks: HashMap::new(),
      depth: depth,
      chunk_size: chunk_size,
      offset: offset,
      noise: noise,
      height_scale: 16.0,
      frequency: 0.0125,
      voxel_scale: 1.0,
      range: 1,
      colors: DEFAULT_COLOR_PALETTE.to_vec(),
      voxel_reuse: voxel_reuse,
    }
  }
}

impl ChunkManager {

  pub fn new(
    depth: u32, 
    voxel_scale: f32, 
    range: u8,
    colors: Vec<[f32; 3]>,  
  ) -> Self {
    let noise = OpenSimplex::new().set_seed(1234);
    let offset = 2;
    let chunk_size = 2_i32.pow(depth) as u32;

    ChunkManager {
      chunks: HashMap::new(),
      depth: depth,
      chunk_size: chunk_size,
      offset: offset,
      noise: noise,
      height_scale: 16.0,
      frequency: 0.0125,
      voxel_scale: voxel_scale,
      range: range,
      colors: colors,
      voxel_reuse: VoxelReuse::new(depth, 3)
    }
  }

  pub fn new_1(
    depth: u32, 
  ) -> Self {
    let noise = OpenSimplex::new().set_seed(1234);
    let offset = 2;
    let chunk_size = 2_i32.pow(depth) as u32;

    ChunkManager {
      chunks: HashMap::new(),
      depth: depth,
      chunk_size: chunk_size,
      offset: offset,
      noise: noise,
      height_scale: 16.0,
      frequency: 0.0125,
      voxel_reuse: VoxelReuse::new(depth, 3),
      ..Default::default()
    }
  }

  pub fn set_voxel(&mut self, pos: &[i64; 3], voxel: u8) -> Chunk {
    let key = voxel_pos_to_key(pos, self.chunk_size);
    let mut chunk = match self.chunks.get(&key) {
      Some(c) => c.clone(),
      // None => ChunkManager::new_chunk_2(
      //   &key, self.depth as u8, 0, self.noise, voxel_by_noise
      // )
      None => Chunk::new(key, self.depth as u8)
    };

    let sizei64 = self.chunk_size as i64;
    let local_x = (pos[0] - (key[0] * sizei64)) as u32;
    let local_y = (pos[1] - (key[1] * sizei64)) as u32;
    let local_z = (pos[2] - (key[2] * sizei64)) as u32;
    
    chunk.octree.set_voxel(local_x, local_y, local_z, voxel);
    self.chunks.insert(key.clone(), chunk.clone());
    chunk
  }

  pub fn get_voxel(&self, pos: &[i64; 3]) -> u8 {
    let key = voxel_pos_to_key(pos, self.chunk_size);
    let octree = match self.chunks.get(&key) {
      Some(c) => &c.octree,
      None =>  {
        return 0
      }
    };

    let sizei64 = self.chunk_size as i64;
    let local_x = pos[0] - (key[0] * sizei64);
    let local_y = pos[1] - (key[1] * sizei64);
    let local_z = pos[2] - (key[2] * sizei64);
    octree.get_voxel(local_x as u32, local_y as u32, local_z as u32)
  }

  pub fn get_voxel_mut(&mut self, pos: &[i64; 3]) -> u8 {
    let key = voxel_pos_to_key(pos, self.chunk_size);
    let octree = match self.chunks.get(&key) {
      Some(c) => &c.octree,
      None =>  { 
        let chunk = ChunkManager::new_chunk(
          &key, self.depth as u8, 0, self.noise, voxel_by_noise
        );
        self.chunks.insert(key, chunk.clone());

        let sizei64 = self.chunk_size as i64;
        let local_x = pos[0] - (key[0] * sizei64);
        let local_y = pos[1] - (key[1] * sizei64);
        let local_z = pos[2] - (key[2] * sizei64);
        return chunk.octree.get_voxel(local_x as u32, local_y as u32, local_z as u32)
      }
    };

    let sizei64 = self.chunk_size as i64;
    let local_x = pos[0] - (key[0] * sizei64);
    let local_y = pos[1] - (key[1] * sizei64);
    let local_z = pos[2] - (key[2] * sizei64);
    octree.get_voxel(local_x as u32, local_y as u32, local_z as u32)
  }


  /**
   * Returns None if the chunk is not loaded containing the coordinate 
   */
  pub fn get_voxel_safe(&self, pos: &[i64; 3]) -> Option<u8> {
    let seamless_size = self.seamless_size();
    let key = voxel_pos_to_key(pos, seamless_size);
    
    let octree = match self.get_octree(&pos) {
      Some(o) => o,
      None => return None
    };

    let sizei64 = seamless_size as i64;
    let local_x = pos[0] - (key[0] * sizei64);
    let local_y = pos[1] - (key[1] * sizei64);
    let local_z = pos[2] - (key[2] * sizei64);

    // println!("key1 {:?} local {} {} {}", key, local_x, local_y, local_z);

    Some(octree.get_voxel(local_x as u32, local_y as u32, local_z as u32))
  }

  fn get_octree(&self, pos: &[i64; 3]) -> Option<&VoxelOctree> {
    let key = &voxel_pos_to_key(pos, self.chunk_size);
    let chunk = match self.get_chunk(key) {
      Some(o) => o,
      None => return None,
    };
    Some(&chunk.octree)
  }




  pub fn seamless_size(&self) -> u32 {
    self.chunk_size - self.offset
  }


  pub fn new_chunk<F>(
    key: &[i64; 3], depth: u8, lod: usize, noise: OpenSimplex, f: F
  ) -> Chunk 
  where F: Fn(i64, i64, i64, i64, OpenSimplex) -> u8 {
    let size = 2_i32.pow(depth as u32) as u32;

    let start_x = key[0] * size as i64;
    let start_y = key[1] * size as i64;
    let start_z = key[2] * size as i64;

    let ground_level = 5;

    let new_octree = VoxelOctree::new(0, depth);
    let mut chunk = Chunk {
      key: key.clone(),
      lod: lod,
      octree: new_octree,
      mode: ChunkMode::None,
      is_default: true,
    };

    let mut has_air = false;
    let mut has_value = false;
    let mut data = Vec::new();

    let start = 0;
    let end = size;
    for octree_x in start..end {
      for octree_y in start..end {
        for octree_z in start..end {
          let x = start_x + octree_x as i64;
          let y = start_y + octree_y as i64;
          let z = start_z + octree_z as i64;

          // let voxel = if y < ground_level { 1 } else { 0 };
          let voxel = f(x, y, z, ground_level, noise) as u32;
          // if voxel == 1 {
          //   println!("voxel {}: {:?}: {} {} {}", voxel, key, x, y, z);
          // }

          data.push([octree_x, octree_y, octree_z, voxel]); 

          // if voxel > 0 {
          //   println!("voxel {} {} {} {}", octree_x, octree_y, octree_z, voxel);
          // }

          /*
            TODO:
              Conditions to determine if Chunk is needed to be rendered and create collider
                Mode:
                  Empty/Air
                  Inner
                  Visible
                Air
                  If all values are 0
                Inner
                  If all values are 1
                Visible
                  ?
          */
          if octree_x <= end - 1
            && octree_y <= end - 1
            && octree_z <= end - 1
          {
            if voxel == 0 {
              has_air = true;
              // println!("Air {} {} {}", octree_x, octree_y, octree_z);
            }
            if voxel == 1 {
              has_value = true;
              // println!("Voxel {} {} {}", octree_x, octree_y, octree_z);
            }
          }
        }
      }
    }

    chunk.octree = VoxelOctree::new_from_3d_array(0, depth, &data, ParentValueType::Lod);
    // chunk.mode = chunk_mode(&chunk.octree);

    /*
      TODO: Have to update mode detector
    */
    if (!has_air && has_value) || (has_air && !has_value) {
      chunk.mode = ChunkMode::Air;  // Should be renamed as empty
    }
    if has_air && has_value {
      chunk.mode = ChunkMode::Loaded;
    }
    // println!("{} {} {}", has_air, has_value, end - 2);
    chunk
  }





  pub fn new_chunk_mut(&mut self, key: &[i64; 3]) -> Chunk {
    let c = ChunkManager::new_chunk(key, self.depth as u8, 0, self.noise, voxel_by_noise);
    self.chunks.insert(*key, c.clone());
    c
  }

  pub fn new_chunk_mut_1(&mut self, key: &[i64; 3], lod: usize) -> Chunk {
    let c = ChunkManager::new_chunk(key, self.depth as u8, lod, self.noise, voxel_by_noise);
    self.chunks.insert(*key, c.clone());
    c
  }




  pub fn chunk_mode(self: &Self, key: &[i64; 3]) -> ChunkMode {
    let chunk = self.chunks.get(key);
    let mut mode = ChunkMode::Unloaded;
    if chunk.is_some() {
      mode = chunk.unwrap().mode;
    }
    mode
  }

  pub fn get_chunk(&self, key: &[i64; 3]) -> Option<&Chunk> {
    /* Later on, implement Spatial Partition or R-trees? */
    self.chunks.get(key)
  }

  pub fn get_chunk_mut(&mut self, key: &[i64; 3]) -> Option<&mut Chunk> {
    /* Later on, implement Spatial Partition or R-trees? */
    self.chunks.get_mut(key)
  }


  /// Refactor: Doesn't make any sense
  pub fn set_chunk(&mut self, key: &[i64; 3], chunk: &Chunk) {
    let c = self.chunks.get(key);
    if c.is_some() {
      if !chunk.is_default {
        self.chunks.insert(key.clone(), chunk.clone());
      }
    } else {
      self.chunks.insert(key.clone(), chunk.clone());
    }
  }

  pub fn remove_chunk(&mut self, key: &[i64; 3]) {
    let chunk_op = self.get_chunk(key);
    if chunk_op.is_some() {
      let chunk = chunk_op.unwrap();
      if chunk.is_default {
        self.chunks.remove(key);
      }
    }
  }

  pub fn len(&self) -> usize {
    self.chunks.len()
  }

  

  pub fn get_adj_chunks(&mut self, key: [i64; 3]) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    let keys = adjacent_keys(&key, self.range as i64, true);
    for key in keys.iter() {
      let res = self.chunks.get(key);
      if res.is_some() {
        chunks.push(res.unwrap().clone());
      }

      if res.is_none() {
        let c = ChunkManager::new_chunk(
          key, self.depth as u8, 0, self.noise, voxel_by_noise
        );
        chunks.push(c.clone());
        self.chunks.insert(*key, c);
      }
    }

    chunks
  }

}

pub fn voxel_by_noise(x: i64, y: i64, z: i64, middle: i64, noise: OpenSimplex) -> u8 {
  let frequency = 0.0125;
  let height_scale = 16.0;
  let fx = (x - middle) as f64 * frequency;
  let fz = (z - middle) as f64 * frequency;
  let noise = noise.get([fx, fz]);
  let elevation = (noise * height_scale) as i64;

  let height = y + elevation;

  // println!("elevation {} height {} middle {}", elevation, height, middle);
  
  // if height < middle {
  //   // println!("height {} y {}", height, y);
  //   return 1;
  // }
  // 0

  if height < middle {
  // if y < middle {
    // println!("y {}", y);
    return 1;
  }
  0
}

#[cfg(test)]
mod tests {
  use crate::{data::{surface_nets::{GridPosition, VoxelReuse}, voxel_octree::VoxelMode}, utils::{get_length, coord_to_index}};
  use super::*;

  #[test]
  fn test_set_and_get_voxel() -> Result<(), String> {
    let mut chunk_manager = ChunkManager::default();

    let start = -10;
    let end = 10;
    let mut new_value = 0;
    
    for x in start..end {
      for y in start..end {
        for z in start..end {
          new_value = if new_value == 255 { 0 } else { new_value + 1 };
          let pos = &[x, y, z];
          chunk_manager.set_voxel(pos, new_value);
        }
      }
    }

    new_value = 0;
    for x in start..end {
      for y in start..end {
        for z in start..end {
          new_value = if new_value == 255 { 0 } else { new_value + 1 };
          let expected = new_value;
          let pos = &[x, y, z];
          let result = chunk_manager.get_voxel(pos);

          assert_eq!(result, expected, "at pos: {:?}", (x, y, z));
        }
      }
    }

    Ok(())
  }

  #[test]
  fn test_set_and_get_voxel_1() -> Result<(), String> {
    let mut chunk_manager = ChunkManager::default();

    let start = -32;
    let end = 32;
    let mut new_value = 0;
    
    for x in start..end {
      for y in start..end {
        for z in start..end {
          new_value = if new_value == 255 { 0 } else { new_value + 1 };
          let pos = &[x, y, z];
          chunk_manager.set_voxel_2(pos, new_value);
        }
      }
    }

    new_value = 0;
    for x in start..end {
      for y in start..end {
        for z in start..end {
          new_value = if new_value == 255 { 0 } else { new_value + 1 };
          let expected = new_value;
          let pos = &[x, y, z];
          let result = chunk_manager.get_voxel_2(pos);

          assert_eq!(result, expected, "at pos: {:?}", (x, y, z));
        }
      }
    }

    Ok(())
  }

  #[test]
  fn test_set_and_get_voxel_2() -> Result<(), String> {
    let mut chunk_manager = ChunkManager::default();

    // let keys = vec![
    //   [0, -1, 0],
    //   [0, 0, 0]
    // ];

    let keys = adjacent_keys(&[0, 0, 0], 1, true);

    for key in keys.iter() {
      let chunk_1 = chunk_manager.new_chunk_mut(key);
      let chunk_2 = ChunkManager::new_chunk_1(
        key, chunk_manager.depth as u8, 0, chunk_manager.noise
      );

      let len = chunk_1.octree.size;
      for x in 0..len {
        for y in 0..len {
          for z in 0..len {
            let voxel_1 = chunk_1.octree.get_voxel(x, y, z);
            let voxel_2 = chunk_2.octree.get_voxel(x, y, z);

            let w_x = key[0] * len as i64 + x as i64;
            let w_y = key[1] * len as i64 + y as i64;
            let w_z = key[2] * len as i64 + z as i64;
            let voxel_3 = chunk_manager.get_voxel_2(&[w_x, w_y ,w_z]);

            assert_eq!(voxel_1, voxel_2, "Not equal");
            assert_eq!(voxel_1, voxel_3, "Not equal");
          }
        }
      }
    }

    Ok(())
  }

  #[test]
  fn test_set_and_get_voxel_3() -> Result<(), String> {
    let mut chunk_manager = ChunkManager::default();

    // let keys = vec![
    //   [0, -1, 0],
    //   [0, 0, 0]
    // ];

    // let keys = adjacent_keys(&[0, 0, 0], 1, true);
    let keys = vec![[0, 0, 0]];

    for key in keys.iter() {
      let chunk_1 = ChunkManager::new_chunk_2(
        key, chunk_manager.depth as u8, 0, chunk_manager.noise, voxel_by_noise
      );

      let size = chunk_1.octree.size;
      let mut voxel_reuse = VoxelReuse::new_1(size);

      let data = chunk_1.octree.compute_mesh2(VoxelMode::SurfaceNets, &chunk_manager, *key, 0);
      println!("data.len() {}", data.indices.len());


      let start_x = key[0] * chunk_1.octree.size as i64;
      let start_y = key[1] * chunk_1.octree.size as i64;
      let start_z = key[2] * chunk_1.octree.size as i64;

      let len = chunk_1.octree.size;
      let ground = 5;
      for x in 0..len {
        for y in 0..len {
          for z in 0..len {
            let voxel_1 = chunk_1.octree.get_voxel(x, y, z);

            let w_x = x as i64 + start_x;
            let w_y = y as i64 + start_y;
            let w_z = z as i64 + start_z;

            let voxel_2 = voxel_by_noise(w_x, w_y, w_z, ground, chunk_manager.noise);
            assert_eq!(voxel_1, voxel_2);

            let index = coord_to_index(x, y, z, 0, size);
            let voxel_3 = voxel_reuse.voxels[index];

            assert_eq!(voxel_3, voxel_1);

            // if voxel_1 == 0 {
              // println!("voxel {}", voxel_1);
            // }


            // let w_x = key[0] * len as i64 + x as i64;
            // let w_y = key[1] * len as i64 + y as i64;
            // let w_z = key[2] * len as i64 + z as i64;
            // let voxel_2 = chunk_manager.get_voxel_2(&[w_x, w_y ,w_z]);

            // assert_eq!(voxel_1, voxel_2, "Not equal");
          }
        }
      }
    }

    Ok(())
  }

  #[test]
  fn test_set_and_get_voxel_in_chunk_manager_custom_algorithm() -> Result<(), String> {
    let ground = 5;
    
    let mut chunk_manager = ChunkManager::default();
    let keys = vec![[0, 0, 0], [0, 0, 1]];
    for key in keys.iter() {

      let chunk = ChunkManager::new_chunk_2(
        key, chunk_manager.depth as u8, 0, chunk_manager.noise,
        |x: i64, y: i64, z: i64, middle: i64, noise: OpenSimplex | {
          if y < ground { 1 } else { 0 }
        }
      );
      chunk_manager.set_chunk(key, &chunk);
    }

    let default_len = chunk_manager.chunk_size as i64;
    let len = 20;
    let ground = 5;
    for x in 0..default_len {
      for y in 0..default_len {
        for z in 0..len {
          let voxel = chunk_manager.get_voxel_2(&[x, y, z]);

          if y < ground {
            assert_eq!(voxel, 1, "at {} {} {}", x, y, z);
          }
        }
      }
    }

    Ok(())
  }




  #[test]
  fn test_chunk_mode() -> Result<(), String> {
    let depth = 4;
    let len = get_length(depth as u8);
    let mut voxels = vec![0; len];
    let size = (2 as u32).pow(depth as u32);
    
    let grid_pos_len = get_length(size as u8 - 1);
    let mut grid_pos = Vec::new();
    for i in 0..grid_pos_len {
      grid_pos.push(GridPosition ::default());
    }

    let mut voxel_reuse = VoxelReuse {
      voxels: voxels,
      grid_pos: grid_pos,
      size: size,
    };

    let chunk_size = 16;
    let mut chunk_manager = ChunkManager::default();

    let color = vec![[0.0, 0.0, 0.0]];

    let keys = adjacent_keys(&[0, 0, 0], 5, true);
    for key in keys.iter() {
      // let chunk = chunk_manager.new_chunk(key, chunk_manager.depth as u8);
      let chunk = ChunkManager::new_chunk(
        key, 
        chunk_manager.depth as u8, 
        chunk_manager.depth as usize, 
        chunk_manager.noise
      );
      let d = chunk.octree.compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut voxel_reuse,
        &color,
        1.0,
        *key,
        0
      );
      if d.indices.len() != 0 {
        assert_eq!(chunk.mode, ChunkMode::Loaded, "key {:?}", key);
      } else {
        assert_eq!(chunk.mode, ChunkMode::Air, "key {:?}", key);
      }
    }

    // let key = [-3, -1, 1];
    // let key = [-5, -1, -2];
    // let chunk = chunk_manager.new_chunk3(&key, chunk_manager.depth as u8);
    // assert_eq!(chunk.mode, ChunkMode::Loaded, "key {:?}", key);

    Ok(())
  }


}



/*
  Need to refactor ChunkManger(Defer)
  Make new features work first
  Then refactor once approved
*/

