extern crate rand;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

pub fn get_plane(sizeX : i32, sizeY : i32) -> Vec<Vertex> {
	let mut toReturn : Vec<Vertex> = Vec::new();
	use rand::distributions::{IndependentSample, Range};
	let mut height_map = Vec::new();
    
	for i in 0..sizeX{
		let mut row : Vec<f32> = Vec::new();
		for j in 0..sizeY{
			let between = Range::new(0, 100);
   		//	let mut rng : f32 = rand::thread_rng() as f32;
   			let n = (rand::thread_rng().gen_range(0, 100) as f32 / 100.0);
   			//let n : f32 = ((between.ind_sample(&mut rng)) / 50.0) as f32;
   			row.push(n);
		}
		height_map.push(row);
	}
	//println!("{:?}", height_map);

	let smoothing_scale_factor : i32 = 4;


	//let mut height_map = Vec::new();
    
	//for i in 0..sizeX * smoothing_scale_factor{
	//	let mut row : Vec<f32> = Vec::new();
	//	for j in 0..sizeY * smoothing_scale_factor{
	//		let mut n : f32 = 0.0;
	//		if(i % smoothing_scale_factor == 0 && j % smoothing_scale_factor == 0){
	//			n = (match height_map_raw.get(i as usize){
	//				Some(x) => {match x.get(i as usize){
	//					Some(y) => y,
	//					None => {let f = 0.0; &f}
	//				},
	//				None => {let z = 0.0; &z}
	//			}
	//			})
				//n = 1.0;
	//		}
   	//		row.push(n);
	//	}
	//	height_map.push(row);
	//}

	println!("{:?}", height_map);

	//sm = scale multiplier
	let sm : f32 = 1.0;
	let vs : f32 = 4.5;
	for i in 0..sizeX{
		for j in 0..sizeY{
			let blank : f32 = 0.0;
			let blank_ref : &f32 = &blank;
			let lu : &f32 = (match height_map.get((i) as usize){Some(v) => {match v.get((j) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let ru : &f32 = (match height_map.get((i + 1) as usize){Some(v) => {match v.get((j) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let ll : &f32 = (match height_map.get((i) as usize){Some(v) => {match v.get((j + 1) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let rl : &f32 = (match height_map.get((i + 1) as usize){Some(v) => {match v.get((j + 1) as usize) { Some(n) => n, _ => blank_ref}}, _ => blank_ref});
			let between = Range::new(0, 100);
 

      		let i : f32 = i as f32;
			let j : f32 = j as f32;
			
			toReturn.push(Vertex { position: [1.0*sm + i*2.0, *rl * vs, 1.0*sm + j*2.0], uv: [ 0.0, 1.0 ] });
			toReturn.push(Vertex { position: [ -1.0*sm + i*2.0, *ll * vs, 1.0*sm + j*2.0], uv: [ 1.0, 1.0 ] });
			toReturn.push(Vertex { position: [ 1.0*sm + i*2.0, *ru * vs, -1.0*sm + j*2.0], uv: [ 0.0, 0.0 ] });
			toReturn.push(Vertex { position: [-1.0*sm + i*2.0, *lu * vs, -1.0*sm + j*2.0], uv: [ 1.0, 0.0] });
			toReturn.push( Vertex { position: [ 1.0*sm + i*2.0, *ru * vs, -1.0*sm + j*2.0], uv: [ 0.0, 0.0] });
			toReturn.push( Vertex { position: [ -1.0*sm + i*2.0, *ll * vs, 1.0*sm + j*2.0], uv: [ 1.0, 1.0 ] });
		}
	}
	return toReturn;
}

pub fn get_sphere(divisionsX : i32, divisionY : i32) -> Vec<Vertex> {
	let vertex1 = Vertex { position: [-1.0, -1.0, -2.0], uv: [ 0.0, 1.0 ] };
	let vertex2 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ] };
	let vertex3 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0 ] };

	let vertex4 = Vertex { position: [1.0, 1.0, -2.0], uv: [ 1.0, 0.0] };
	let vertex5 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0] };
	let vertex6 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ] };
	vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]
}