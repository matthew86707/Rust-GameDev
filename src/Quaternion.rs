use std::ops;
use ::nalgebra;
use ::nalgebra::Vector3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn identity() -> Quaternion {
        Quaternion {
        x : 0.0,
        y : 0.0,
        z : 0.0,
        w : 1.0
        }
    }

    pub fn get_conjugate(&self) -> Quaternion {
         let v_quat = Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: 0.0
        };
        v_quat
    }

    pub fn transform_vector(self, v: Vector3<f32>) -> Vector3<f32> {
        let v_quat = Quaternion {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        };
        let q = self * v_quat * self.get_conjugate();
        Vector3::new(q.x, q.y, q.z)
    }

    pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> Quaternion {
        use std::f32;
        let mut cy : f32 = f32::cos(yaw * 0.5);
	    let mut sy : f32 = f32::sin(yaw * 0.5);
	    let mut cr : f32 = f32::cos(roll * 0.5);
	    let mut sr : f32 = f32::sin(roll * 0.5);
	    let mut cp : f32 = f32::cos(pitch * 0.5);
	    let mut sp : f32 = f32::sin(pitch * 0.5);

        Quaternion {
            w : cy * cr * cp + sy * sr * sp,
            z : cy * sr * cp - sy * cr * sp,
            x : cy * cr * sp + sy * sr * cp,
            y : sy * cr * cp - cy * sr * sp
        }
    }

    pub fn from_axis_angle(x: f32, y: f32, z: f32, angle: f32) -> Quaternion {
        Quaternion {
            x : x * f32::sin(angle / 2.0),
            y : y * f32::sin(angle / 2.0),
            z : z * f32::sin(angle / 2.0),
            w : f32::cos(angle / 2.0)
        }
    }
       
    pub fn len(&self) -> f32 {
        use std::f32;
        return (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt();
    }
    
    pub fn normalize(&mut self) {
        let mut length : f32 = self.len();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
        self.w = self.w / length;
    }

    pub fn into_matrix(self) -> nalgebra::core::Matrix4<f32> {
        use std::f32;
        let mut matrix_array : [[f32; 4];4] = [[0.0; 4];4];
        let w = self.w;
        let x = self.x;
        let y = self.y;
        let z = self.z;

        matrix_array[0][0] = 1.0 - 2.0 * y.powf(2.0) - 2.0 * z.powf(2.0);
        matrix_array[1][1] = 1.0 - 2.0 * x.powf(2.0) - 2.0 * z.powf(2.0);
        matrix_array[2][2] = 1.0 - 2.0 * x.powf(2.0) - 2.0 * y.powf(2.0);

        matrix_array[1][0] = 2.0 * x * y - 2.0 * z * w;
        matrix_array[2][0] = 2.0 * x * z + 2.0 * y * w;

        matrix_array[0][1] = 2.0 * x * y + 2.0 * z * w;
        matrix_array[2][1] = 2.0 * y * z - 2.0 * x * w;

        matrix_array[0][2] = 2.0 * x * z - 2.0 * y * w;
        matrix_array[1][2] = 2.0 * y * z + 2.0 * x * w;

        matrix_array[3][3] = 1.0;

        return matrix_array.into();
    
    }

    pub fn slerp(&self, dst: Quaternion, t: f32) -> Quaternion {
        unimplemented!()
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        let mut q = Quaternion {
            w : (rhs.w * self.w - rhs.x * self.x - rhs.y * self.y - rhs.z * self.z),
            x : (rhs.w * self.x + rhs.x * self.w - rhs.y * self.z + rhs.z * self.y),
            y : (rhs.w * self.y + rhs.x * self.w + rhs.y * self.w - rhs.z * self.x),
            z : (rhs.w * self.z - rhs.x * self.y + rhs.y * self.x + rhs.z * self.w)
        };
        q.normalize();
        return q;
    }
}

impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        *self = Quaternion {
            w : (rhs.w * self.w - rhs.x * self.x - rhs.y * self.y - rhs.z * self.z),
            x : (rhs.w * self.x + rhs.x * self.w - rhs.y * self.z + rhs.z * self.y),
            y : (rhs.w * self.y + rhs.x * self.w + rhs.y * self.w - rhs.z * self.x),
            z : (rhs.w * self.z - rhs.x * self.y + rhs.y * self.x + rhs.z * self.w)
        };
        self.normalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::f32::consts::PI;

    const EPSILON: f32 = 1.0e-6;

    // Check if two floats are approximately equal.
    fn assert_feq(a: f32, b: f32) {
        if (a-b).abs() >= EPSILON {
            panic!("Not equal: {} {}", a, b);
        }
    }

    // Check if two Quaternions are approximately equal.
    fn assert_quat_eq(a: Quaternion, b: Quaternion) {
        if (a.x-b.x).abs() >= EPSILON || (a.y-b.y).abs() >= EPSILON || (a.z- b.z).abs() >= EPSILON || (a.w-b.w).abs() >= EPSILON {
            panic!("Not equal: {:?} {:?}", a, b);
        }
    }

    // Check if two Quaternion represent approximately the same rotation.
    // Both q and -q represent the same rotation.
    fn assert_rot_eq(a: Quaternion, b: Quaternion) {
        if (a.x-b.x).abs() >= EPSILON || (a.y-b.y).abs() >= EPSILON || (a.z- b.z).abs() >= EPSILON || (a.w-b.w).abs() >= EPSILON {
            if (a.x+b.x).abs() >= EPSILON || (a.y+b.y).abs() >= EPSILON || (a.z+b.z).abs() >= EPSILON || (a.w+b.w).abs() >= EPSILON {
                panic!("Not equal: {:?} {:?}", a, b);
            }
        }
    }

    #[test]
    fn test_identity() {
        assert_eq!(Quaternion::identity(), Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });
    }

    #[test]
    fn test_len() {
        assert_eq!(Quaternion::identity().len(), 1.0);
        assert_eq!(Quaternion { x: 2.0, y: 2.0, z: 2.0, w: 2.0 }.len(), 4.0);

        // Rotations should be unit quaternions.
        let q = Quaternion::from_euler_angles(0.1, 0.4, 123.0);
        if (q.len() - 1.0).abs() >= EPSILON {
            panic!("Incorrect length");
        }
    }

    #[test]
    fn test_normalize() {
        let mut q = Quaternion { x: 2.0, y: -2.0, z: 2.0, w: -2.0 };
        q.normalize();
        assert_feq(q.len(), 1.0);
        assert_quat_eq(q, Quaternion { x: 0.5, y: -0.5, z: 0.5, w: -0.5 });
    }

    #[test]
    fn test_euler_angles() {
        let q = Quaternion::from_euler_angles(0.0, 0.0, PI);
        assert_rot_eq(q, Quaternion { x: 0.0, y: 1.0, z: 0.0, w: 0.0 });

        let q = Quaternion::from_euler_angles(0.0, PI * 2.0, 0.0);
        assert_rot_eq(q, Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });

        let p = Quaternion::from_euler_angles(PI / 2.0, PI / 2.0, PI / 2.0);
        let q =  Quaternion::from_euler_angles(0.0, PI / 2.0, 0.0);
        assert_rot_eq(p, q);
    }

    #[test]
    fn test_axis_angle() {
        let q = Quaternion::from_axis_angle(1.0, 0.0, 0.0, PI * 3.0);
        assert_rot_eq(q, Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 });

        let sqrt2_2 = 0.707107;
        let q = Quaternion::from_axis_angle(0.0, sqrt2_2, -sqrt2_2, PI / 2.0);
        assert_rot_eq(q, Quaternion { x: 0.0, y: 0.5, z: -0.5, w: sqrt2_2 });

        // Rotating 360 around any axis should be equivalent.
        let p = Quaternion::from_axis_angle(1.0, 0.0, 0.0, PI * 2.0);
        let q = Quaternion::from_axis_angle(sqrt2_2, sqrt2_2, 0.0, -PI * 4.0);
        assert_rot_eq(p, q);
    }

    #[test]
    fn test_mul() {
        let id = Quaternion::identity();
        let zero = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        assert_quat_eq(id*zero, zero);
        assert_quat_eq(id*id, id);

        let p = Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
        let q = Quaternion { x: -1.0, y: 0.0, z: 0.0, w: 0.0 };
        assert_quat_eq(q*id, q);
        assert_quat_eq(p*q, id);

        let mut q = Quaternion::identity();
        let p = Quaternion::from_axis_angle(1.0, 0.0, 0.0, PI);
        q *= p;
        assert_rot_eq(q, p);
        q *= p;
        assert_rot_eq(q, id);

        let q = Quaternion::from_axis_angle(0.0, -1.0, 0.0, 0.2);
        let p = Quaternion::from_euler_angles(0.5, 0.1, 0.9);
        assert_rot_eq(p*q, Quaternion { x: 0.17043722, y: 0.3202073, z: 0.185343, w: 0.9132724 });
    }

    #[test]
    fn test_into_matrix() {
        fn assert_matrix_eq(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) {
            for i in 0..4 {
                for j in 0..4 {
                    assert_feq(a[i][j], b[i][j]);
                }
            }
        }

        let q = Quaternion::identity();
        let m: [[f32; 4]; 4] = q.into_matrix().into();
        assert_matrix_eq(m, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]);

        // Upside down rotation
        let q = Quaternion::from_axis_angle(0.0, 1.0, 0.0, PI/2.0);
        let m: [[f32; 4]; 4] = q.into_matrix().into();
        assert_matrix_eq(m, [[0.0, 0.0, -1.0, 0.0], [0.0, 1.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0]]);
    }

    #[test]
    fn test_slerp() {
        let p = Quaternion::identity();
        let q = Quaternion::from_euler_angles(0.0, 0.0, 5.0 * PI);
        let r = Quaternion::from_axis_angle(0.0, 1.0, 0.0, PI/2.0);
        assert_rot_eq(p.slerp(q, 0.5), r);

        let p = Quaternion::from_euler_angles(0.2, 0.4, 3.1 * PI);
        let p = Quaternion::from_euler_angles(0.9, 0.2, -120.0);
        assert_rot_eq(p.slerp(q, 0.9), Quaternion { x: 0.0060857176, y: 0.9921862, z: -0.05777763, w: -0.11041405 });
    }
}