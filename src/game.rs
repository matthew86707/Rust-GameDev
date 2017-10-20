pub struct Game<'a> {
	pub world: World<'a>
}

impl<'a> Game<'a> {
	pub fn new(air_block: Block<'a>) -> Game<'a> {
		Game {
			world: create_world(air_block)
		}
	}
}

pub struct World<'a> {
	chunks: [[Chunk<'a>; 16]; 16]
}

impl<'a> World<'a> {
	pub fn get_block(&mut self, x: u32, y: u32, z: u32) -> Block {
		self.chunks[(x >> 4) as usize][(y >> 4) as usize].blocks[(x & 15) as usize][(z & 15) as usize][y as usize]
	}

	pub fn set_block(&mut self, x: u32, y: u32, z: u32, block: Block<'a>) {
		self.chunks[(x >> 4) as usize][(z >> 4) as usize].blocks[(x & 15) as usize][(z & 15) as usize][y as usize] = block;
	}
}

use ndarray::Array3;

pub struct Chunk<'a> {
	//blocks: [[[Block<'a>; 255]; 16]; 16]
	blocks: Array3
}

use glium;

pub struct Block<'a> {
	pub image: glium::texture::RawImage2d<'a, u8>
}

use utils;

impl<'a> Block<'a> {
	pub fn new(img: &str) -> Block<'static> {
		Block {
			image: utils::load_image_from_file(img)
		}
	}

	pub fn get_texture_2d(&self, display: &mut glium::Display) -> glium::texture::Texture2d {
		glium::texture::Texture2d::new(display, self.image).unwrap()
	}

	pub fn get_vertex_buffer(&self, display: &mut glium::Display) -> glium::VertexBuffer<Vertex> {
		let vertices = vec![Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0] }, Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0] }];
		glium::VertexBuffer::new(display, &vertices).unwrap()
	}

	pub fn get_index_buffer(&self, display: &mut glium::Display) -> glium::IndexBuffer<u16> {
		let indices = vec![0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7, 8, 9, 10, 10, 9, 11, 12, 13, 14, 14, 13, 15, 16, 17, 18, 18, 17, 19, 20, 21, 22, 22, 21, 23];
		glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap()
	}
}

pub struct Blocks<'a> {
	pub block_stone: Block<'a>
}

impl<'a> Blocks<'a> {
	pub fn new() -> Blocks<'a> {
		Blocks {
			block_stone: Block::new("models/stone.png")
		}
	}
}

pub fn create_world<'a>(air_block: Block<'a>) -> World<'a> {
	let mut chunk_array: [[Chunk<'a>; 16]; 16];

	for x in 0..16 {
		for z in 0..16 {
			chunk_array[x][z] = create_chunk(air_block);
		}
	}

	World {
		chunks: chunk_array
	}
}

fn create_chunk<'a>(air_block: Block<'a>) -> Chunk<'a> {
	let mut block_array: [[[Block<'a>; 255]; 16]; 16];

	for x in 0..16 {
		for z in 0..16 {
			for y in 0..256 {
				block_array[x][z][y] = air_block;
			}
		}
	}

	Chunk {
		blocks: block_array
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            position: [0.0; 3],
            uv: [0.0; 2]
        }
    }
}