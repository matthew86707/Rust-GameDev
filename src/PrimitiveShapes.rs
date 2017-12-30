extern crate rand;
use rand::Rng;

#[derive(Copy, Clone)]
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

pub fn get_plane(sizeX : i32, sizeY : i32) -> Vec<Vertex> {
	let mut toReturn : Vec<Vertex> = Vec::new();
	use rand::distributions::{IndependentSample, Range};
	let mut height_map_raw = Vec::new();
    
	for i in 0..sizeX{
		let mut row : Vec<f32> = Vec::new();
		for j in 0..sizeY{
			let between = Range::new(0, 100);
   		//	let mut rng : f32 = rand::thread_rng() as f32;
   			let n = (rand::thread_rng().gen_range(0, 200) as f32 / 100.0);
   			//let n : f32 = ((between.ind_sample(&mut rng)) / 50.0) as f32;
   			row.push(n - 1.0);
		}
		height_map_raw.push(row);
	}
	//println!("{:?}", height_map);

	let smoothing_scale_factor : i32 = 4;


	let mut height_map = Vec::new();
    
	for i in 0..sizeX * smoothing_scale_factor{
		let mut row : Vec<f32> = Vec::new();
		for j in 0..sizeY * smoothing_scale_factor{
			//if(i % smoothing_scale_factor == 0 && j % smoothing_scale_factor == 0){
			//	row.push(get_float_from_vec_map(&mut height_map_raw, i, j));
			//}else{
				//row.push(0.0);
				// println!("-----New Vertex-----");
				 let mut xCoord : f32 = (i as f32 / smoothing_scale_factor as f32);
				 let mut yCoord : f32 = (j as f32 / smoothing_scale_factor as f32);
				// println!("xCoord {}", xCoord);
				// println!("yCoord {}", yCoord);
				let mut xCoordLowerBlend : f32 = (((xCoord as i32) as f32) + 1.0) - xCoord;
				 let mut yCoordLowerBlend : f32 = (((yCoord as i32) as f32) + 1.0) - yCoord;
				let mut xCoordUpperBlend : f32 = xCoord - ((xCoord as i32) as f32);
				 let mut yCoordUpperBlend : f32 = yCoord - ((yCoord as i32) as f32);
				// println!("xCoordUpperBlendAmount {}", xCoordUpperBlend);
				// println!("xCoordLowerBlendAmount {}", xCoordLowerBlend);
				// println!("yCoordUpperBlendAmount {}", yCoordUpperBlend);
				// println!("yCoordLowerBlendAmount {}", yCoordLowerBlend);
				let mut upperXUpperY : f32 = get_float_from_vec_map(&mut height_map_raw, xCoord.ceil() as i32, yCoord.ceil() as i32);
				let mut lowerXUpperY : f32 = get_float_from_vec_map(&mut height_map_raw, xCoord.floor() as i32, yCoord.ceil() as i32);
				let mut upperXLowerY : f32 = get_float_from_vec_map(&mut height_map_raw, xCoord.ceil() as i32, yCoord.floor() as i32);
				let mut lowerXLowerY : f32 = get_float_from_vec_map(&mut height_map_raw, xCoord.floor() as i32, yCoord.floor() as i32);
				// println!("{} {}", lowerXUpperY, upperXUpperY);
				// println!("{} {}", lowerXLowerY, upperXLowerY);
				let mut ourValueX : f32 = (xCoordUpperBlend * ((upperXLowerY + upperXUpperY) / 2.0))  + (xCoordLowerBlend * ((lowerXLowerY + lowerXUpperY) / 2.0));
				let mut ourValueY : f32 = (yCoordUpperBlend * ((upperXUpperY + lowerXUpperY) / 2.0))  + (yCoordLowerBlend * ((lowerXLowerY + upperXLowerY) / 2.0));
				let mut ourValue : f32 = 0.0;



				//if(i % smoothing_scale_factor == 0 && j % smoothing_scale_factor == 0){
				//	ourValue = get_float_from_vec_map(&mut height_map_raw, i, j);
				//}
				//if(i % smoothing_scale_factor == 0){
				//	ourValue = (ourValueY);
				//}else if (j % smoothing_scale_factor == 0){
				//	ourValue = (ourValueX);
				//}else{
				ourValue = (ourValueX + ourValueY) / 2.0;
				//}
				//println!("ourValue {}", ourValue);
				row.push(ourValue);
			}
			height_map.push(row);
		}
		//height_map.push(row);
	
		

	//println!("{:?}", height_map);

	//sm = scale multiplier
	let sm : f32 = 1.0;
	let vs : f32 = 5.5;
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

pub fn get_sphere(divisionsX : i32, divisionY : i32) -> Vec<Vertex> {
	let vertex1 = Vertex { position: [-1.0, -1.0, -2.0], uv: [ 0.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
	let vertex2 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
	let vertex3 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0 ], normal: [0.0, 0.0, 0.0] };

	let vertex4 = Vertex { position: [1.0, 1.0, -2.0], uv: [ 1.0, 0.0], normal: [0.0, 0.0, 0.0] };
	let vertex5 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0], normal: [0.0, 0.0, 0.0] };
	let vertex6 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
	vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]
}