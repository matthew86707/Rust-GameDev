extern crate glium;
extern crate image;
extern crate nalgebra;

const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE : f32 = 10000.0;

use Quaternion::Quaternion;

pub struct Camera{
	
	pub rotation: Quaternion,
	pub position: nalgebra::Vector3<f32>,
	pub transform: [[f32; 4]; 4],
    pub rotation_scale : f32,
    pub current_velocity : nalgebra::Vector3<f32>
}

impl Camera{
	pub fn new() -> Camera{

 let mut translation: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.0, 0.0);

		let mut translation_matrix : nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut scale_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let transform = translation_matrix * rotation_matrix * scale_matrix;
       // let transform = scale_matrix * rotation_matrix * translation_matrix;
        let transform: [[f32; 4]; 4] = transform.into();
		Camera{
			
			position: nalgebra::Vector3::new(0.0, 0.0, 0.0),
			rotation: Quaternion::identity(),
			transform : transform,
            current_velocity : nalgebra::Vector3::new(0.0, 0.0, 0.0),
            rotation_scale : 1.0
		}
	}

    pub fn update_position(&mut self){
        self.position += self.current_velocity;
    }
	
    pub fn set_rotation_scale(&mut self, scale : f32){
        self.rotation_scale = scale;
    }

	pub fn translate(&mut self, translation: nalgebra::Vector3<f32>) {
		self.position += translation;
	}

    pub fn add_velocity(&mut self, force : nalgebra::Vector3<f32>) {
        self.current_velocity += force;
    }

	pub fn rotate(&mut self, rotation: nalgebra::Vector3<f32>) {
        let mut rotation_x = Quaternion::from_axis_angle(0.0, 0.0, 1.0, rotation.x);
        let mut rotation_y = Quaternion::from_axis_angle(0.0, 1.0, 0.0, rotation.y);

        let mut v = Vector3::new(1.0, 0.0, 0.0);
        v = rotation_y.transform_vector(v);
        let mut rotation_z = Quaternion::from_axis_angle(v.x, v.y, v.z, rotation.z);
        
		
        self.rotation *= rotation_y;
        self.rotation *= rotation_z;
	}

	pub fn get_view_matrix(&self, should_translate : bool) -> [[f32; 4]; 4] {
		let mut translation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut translation_point_matrix: nalgebra::Vector4<f32> = nalgebra::Vector4::new(100.0, 0.0, 0.0, 0.0);
        let mut rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        if(should_translate){

        translation_matrix[(0, 3)] = self.position[0];
        translation_matrix[(1, 3)] = self.position[1];
        translation_matrix[(2, 3)] = self.position[2];

        }

       ((translation_matrix) * self.rotation.into_matrix()).into()      
        
	}

    pub fn get_view_matrix_as_matrix(&self) -> nalgebra::Matrix4<f32> {
        let mut translation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        translation_matrix[(0, 3)] = self.position[0];
        translation_matrix[(1, 3)] = self.position[1];
        translation_matrix[(2, 3)] = self.position[2];

        let mut rotation_matrix_z: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut rotation_matrix_y: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut rotation_matrix_x: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        (translation_matrix) * self.rotation.into_matrix()
    }

    pub fn forward(&self) -> nalgebra::Vector3<f32> {
        let mut point = nalgebra::Vector4::new(0.0, 0.0, -1.0, 0.0);
        point = self.get_view_matrix_as_matrix() * point;

        nalgebra::Vector3::new(point[0], point[1], point[2])
    }

    pub fn right(&self) -> nalgebra::Vector3<f32> {
        let mut point = nalgebra::Vector4::new(1.0, 0.0, 0.0, 0.0);
        point = self.get_view_matrix_as_matrix() * point;

        nalgebra::Vector3::new(point[0], point[1], point[2])
    }

pub fn create_projection_matrix(&mut self, fov: f32, screen_size: (u32, u32)) -> nalgebra::Matrix4<f32> {
    let aspect_ratio: f32 = screen_size.0 as f32 / screen_size.1 as f32;
    let y_scale = (1.0 / f32::tan(f32::to_radians(fov / 2.0))) * aspect_ratio;
    let x_scale = y_scale / aspect_ratio;
    let frustum_length = FAR_PLANE - NEAR_PLANE;

    let mut matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    matrix[(0, 0)] = x_scale;
    matrix[(1, 1)] = y_scale;
    matrix[(2, 2)] = -((FAR_PLANE + NEAR_PLANE) / frustum_length);
    matrix[(3, 2)] = -1.0;
    matrix[(2, 3)] = -((2.0 * NEAR_PLANE * FAR_PLANE) / frustum_length);
    matrix[(3, 3)] = 0.0;

    matrix
}
}
