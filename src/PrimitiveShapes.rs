extern crate rand;
extern crate noise;

use rand::Rng;

use ncollide::shape::Triangle;
use nalgebra::geometry::Point;
use ncollide::shape::Triangle3;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

fn get_float_from_vec_map(vector : &mut Vec<Vec<f32>>, x : i32, y : i32) -> f32 {
	match vector.get(x as usize){
		Some(x1) => {
			match x1.get(y as usize){
				Some(y1) => {return *y1},
				None => {return 0.0 as f32;}
			}
		},
		None => {return 0.0 as f32}
	}
}

pub fn get_sphere(rings : i32, ring_divisions : i32, apply_noise : bool, generate_collision : bool, collisionTriangles : &mut Vec<Triangle3<f32>>) -> Vec<Vertex> {
	let mut toReturn : Vec<Vertex> = Vec::new();
	use rand::distributions::{IndependentSample, Range};
    use noise::{NoiseModule, Perlin};
    use noise::Seedable;
	let perlin = Perlin::new();
	perlin.set_seed(160);

use std;
let mut max_ring_radius : f32 = 0.0;
let mut ring_x : f32 = -1.0;
let d_ring : f32 = 2.0 / (rings - 1) as f32;
	for i in 0..rings{
	
		let mut radius : f32 = 30.0;

		let mut ring_radius : f32 = (1.0 - (ring_x).powf(2.0)).sqrt() * radius;
		let mut lower_ring_radius : f32 = (1.0 - (ring_x + d_ring).powf(2.0)).sqrt() * radius;

		println!("Ring Radius : {}", ring_radius);

		if ring_radius > max_ring_radius {
			max_ring_radius = ring_radius;
		}
		let mut angle : f32 = 0.0;
		let d_angle : f32 = (std::f32::consts::PI * 2.0) / ring_divisions as f32;
		for j in 0..ring_divisions{

			
			use nalgebra::core::Vector3;

			
			let mut next_angle : f32 = angle + d_angle;

			let j : f32 = j as f32;

			let mut noise_strength : f32 = 0.11566;

		

			let mut one : [f32; 3] = [ring_x * radius, angle.cos() * ring_radius, angle.sin() * ring_radius];
			let mut two : [f32; 3] = [(ring_x + d_ring) * radius, next_angle.cos() * lower_ring_radius, next_angle.sin() * lower_ring_radius];
			let mut three : [f32; 3] = [(ring_x + d_ring) * radius, angle.cos() * lower_ring_radius, angle.sin() * lower_ring_radius];
			let mut four : [f32; 3] = [ring_x * radius, next_angle.cos() * ring_radius, next_angle.sin() * ring_radius];

			let i : f32 = i as f32;

		if apply_noise {
			one = (Vector3::new(one[0], one[1], one[2]) + (perlin.get([i * noise_strength, j * noise_strength]) * 1.0 * Vector3::new(one[0], one[1], one[2]).normalize())).into();
			two = (Vector3::new(two[0], two[1], two[2]) + (perlin.get([(i + 1.0) * noise_strength, (j + 1.0) * noise_strength]) * 1.0 * Vector3::new(two[0], two[1], two[2]).normalize())).into();
			three = (Vector3::new(three[0], three[1], three[2]) + (perlin.get([(i + 1.0) * noise_strength, j * noise_strength]) * 1.0 * Vector3::new(three[0], three[1], three[2]).normalize())).into();
			four = (Vector3::new(four[0], four[1], four[2]) + (perlin.get([i * noise_strength, (j + 1.0) * noise_strength]) * 1.0 * Vector3::new(four[0], four[1], four[2]).normalize())).into();
		}

			let mut triangleOneNormal : [f32; 3] = [0.0, 0.0, 0.0];
			let mut triangleTwoNormal : [f32; 3] = [0.0, 0.0, 0.0];

			let point1A : Vector3<f32> = Vector3::new(one[0], one[1], one[2]);
			let point1B : Vector3<f32> = Vector3::new(two[0], two[1], two[2]);
			let point1C : Vector3<f32> = Vector3::new(three[0], three[1], three[2]);

			let U : Vector3<f32> = point1B - point1A;
			let V : Vector3<f32> = point1C - point1A;

			triangleOneNormal = U.cross(&V).into();

			let point2A : Vector3<f32> = Vector3::new(two[0], two[1], two[2]);
			let point2B : Vector3<f32> = Vector3::new(one[0], one[1], one[2]);
			let point2C : Vector3<f32> = Vector3::new(four[0], four[1], four[2]);

			let X : Vector3<f32> = point1B - point1A;
			let Y : Vector3<f32> = point1C - point1A;

			triangleTwoNormal = X.cross(&Y).into();

			
			toReturn.push(Vertex { position: one, uv: [ 0.0, 1.0 ], normal : triangleOneNormal });
			toReturn.push(Vertex { position: two, uv: [ 1.0, 1.0 ], normal : triangleOneNormal });
			toReturn.push(Vertex { position: three, uv: [ 0.0, 0.0 ], normal : triangleOneNormal });

			toReturn.push(Vertex { position: two, uv: [ 1.0, 0.0], normal : triangleTwoNormal });
			toReturn.push( Vertex { position: one, uv: [ 0.0, 0.0], normal : triangleTwoNormal });
			toReturn.push( Vertex { position: four, uv: [ 1.0, 1.0 ], normal : triangleTwoNormal });

			if generate_collision {
				collisionTriangles.push(Triangle3::<f32>::new(Point::from_coordinates(Vector3::<f32>::new(one[0], one[1], one[2])), Point::from_coordinates(Vector3::<f32>::new(two[0], two[1], two[2])), Point::from_coordinates(Vector3::<f32>::new(three[0], three[1], three[2]))));
				collisionTriangles.push(Triangle3::<f32>::new(Point::from_coordinates(Vector3::<f32>::new(two[0], two[1], two[2])), Point::from_coordinates(Vector3::<f32>::new(one[0], one[1], one[2])), Point::from_coordinates(Vector3::<f32>::new(four[0], four[1], four[2]))));
				//collisionTriangles.push(Triangle::new(two, one, four));
			}

			angle += d_angle;

		}

		ring_x += d_ring;
	}
	println!("Radius : {}", max_ring_radius);
	return toReturn;
}


pub fn get_plane(sizeX : i32, sizeY : i32, world_seed : i32) -> Vec<Vertex> {
	let mut toReturn : Vec<Vertex> = Vec::new();
	use rand::distributions::{IndependentSample, Range};
	let mut height_map_raw = Vec::new();
    use noise::{NoiseModule, Perlin};
    use noise::Seedable;
	let perlin = Perlin::new();
	let perlin_macro = Perlin::new();

	perlin.set_seed(world_seed as usize);
	perlin_macro.set_seed((world_seed + 1) as usize);
	
	for i in 0..sizeX{
		let mut row : Vec<f32> = Vec::new();
		for j in 0..sizeY{

   			row.push(perlin.get([((i as f32) / 80.0) +  0.1, ((j as f32) / 80.0) +  0.1]) * perlin_macro.get([((i as f32) / 180.0) +  0.1, ((j as f32) / 180.0) +  0.1]));
		}
		height_map_raw.push(row);
	}

	let mut height_map = height_map_raw;
    

	//sm = scale multiplier
	let sm : f32 = 1.0;
	let vs : f32 = 50.5;
	for i in 0..sizeX{
		for j in 0..sizeY{
			let blank : f32 = 0.0;
			let blank_ref : &f32 = &blank;
			let lu : &f32 = (match height_map.get((i) as usize){Some(v) => {match v.get((j) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let ru : &f32 = (match height_map.get((i + 1) as usize){Some(v) => {match v.get((j) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let ll : &f32 = (match height_map.get((i) as usize){Some(v) => {match v.get((j + 1) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let rl : &f32 = (match height_map.get((i + 1) as usize){Some(v) => {match v.get((j + 1) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let between = Range::new(0, 100);
 
			use nalgebra::core::Vector3;
			

      		let i : f32 = i as f32;
			let j : f32 = j as f32;

			let mut triangleOneNormal : [f32; 3] = [0.0, 0.0, 0.0];
			let mut triangleTwoNormal : [f32; 3] = [0.0, 0.0, 0.0];

			let point1A : Vector3<f32> = Vector3::new(1.0*sm + i*2.0, *rl * vs, 1.0*sm + j*2.0);
			let point1B : Vector3<f32> = Vector3::new(-1.0*sm + i*2.0, *ll * vs, 1.0*sm + j*2.0);
			let point1C : Vector3<f32> = Vector3::new(1.0*sm + i*2.0, *ru * vs, -1.0*sm + j*2.0);

			let U : Vector3<f32> = point1B - point1A;
			let V : Vector3<f32> = point1C - point1A;

			triangleOneNormal = U.cross(&V).into();

			let point2A : Vector3<f32> = Vector3::new(-1.0*sm + i*2.0, *lu * vs, -1.0*sm + j*2.0);
			let point2B : Vector3<f32> = Vector3::new(1.0*sm + i*2.0, *ru * vs, -1.0*sm + j*2.0);
			let point2C : Vector3<f32> = Vector3::new( -1.0*sm + i*2.0, *ll * vs, 1.0*sm + j*2.0);

			let X : Vector3<f32> = point1B - point1A;
			let Y : Vector3<f32> = point1C - point1A;

			triangleTwoNormal = X.cross(&Y).into();


			
			toReturn.push(Vertex { position: [1.0*sm + i*2.0, *rl * vs, 1.0*sm + j*2.0], uv: [ 0.0, 1.0 ], normal : triangleOneNormal });
			toReturn.push(Vertex { position: [ -1.0*sm + i*2.0, *ll * vs, 1.0*sm + j*2.0], uv: [ 1.0, 1.0 ], normal : triangleOneNormal });
			toReturn.push(Vertex { position: [ 1.0*sm + i*2.0, *ru * vs, -1.0*sm + j*2.0], uv: [ 0.0, 0.0 ], normal : triangleOneNormal });

			toReturn.push(Vertex { position: [-1.0*sm + i*2.0, *lu * vs, -1.0*sm + j*2.0], uv: [ 1.0, 0.0], normal : triangleTwoNormal });
			toReturn.push( Vertex { position: [ 1.0*sm + i*2.0, *ru * vs, -1.0*sm + j*2.0], uv: [ 0.0, 0.0], normal : triangleTwoNormal });
			toReturn.push( Vertex { position: [ -1.0*sm + i*2.0, *ll * vs, 1.0*sm + j*2.0], uv: [ 1.0, 1.0 ], normal : triangleTwoNormal });
		}
	}
	return toReturn;
}

