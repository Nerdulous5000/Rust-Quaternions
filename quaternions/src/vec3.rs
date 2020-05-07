use f64;
use super::quaternion::Quaternion;


#[allow(dead_code)]
pub struct Vec3 {
    _x: f64,
    _y: f64,
    _z: f64,

    _magnitude: f64,
}

pub struct Vec3Builder {
    _x: f64,
    _y: f64,
    _z: f64,
}

impl Vec3Builder {
    pub fn x(mut self, val: f64) -> Vec3Builder {
        self._x = val;
        self
    }
    pub fn y(mut self, val: f64) -> Vec3Builder  {
        self._y = val;
        self
    }
    pub fn z(mut self, val: f64) -> Vec3Builder  {
        self._z = val;
        self
    }

    pub fn build(&self) -> Vec3 {
        let mag = (self._x.powi(2) + self._y.powi(2) + self._z.powi(2)).sqrt();

        Vec3 {
            _x: self._x,
            _y: self._y,
            _z: self._z,
            _magnitude: mag,
        }
    }
}

impl Vec3 {
    pub fn init() -> Vec3Builder {
        Vec3Builder {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0
        }
    }
    pub fn x(&self) -> f64 {
        self._x
    }
    pub fn y(&self) -> f64 {
        self._y
    }
    pub fn z(&self) -> f64 {
        self._z
    }


    pub fn magnitude(&self) -> f64{
        self._magnitude
    }

    pub fn rotate(&self, q: Quaternion) -> Vec3 {
        let p = Quaternion::init_by_wxyz()
        .x(self._x)
        .y(self._y)
        .z(self._z)
        .normalized(false)
        .build();

        let rotated_quat = q.clone() * p * q.conjugate();
        let rotated_vec = rotated_quat.as_vec3();

        Vec3 {
            _x: rotated_vec.x(),
            _y: rotated_vec.y(),
            _z: rotated_vec.z(),
            _magnitude: self._magnitude,
        }
    }
}
