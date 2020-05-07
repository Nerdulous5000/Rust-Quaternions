use std::ops::Mul;
use super::vec3::Vec3;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Quaternion {
    _w: f64,
    
    _x: f64,
    _y: f64,
    _z: f64,

    _norm: f64,
}

pub struct QuaternionBuilderByAxisAngle {
    _x: f64,
    _y: f64,
    _z: f64,

    _rot: f64,
    _normalized: bool,
}

impl QuaternionBuilderByAxisAngle {

    pub fn x(mut self, val: f64) -> QuaternionBuilderByAxisAngle {
        self._x = val;
        self
    }
    pub fn y(mut self, val: f64) -> QuaternionBuilderByAxisAngle {
        self._y = val;
        self
    }
    pub fn z(mut self, val: f64) -> QuaternionBuilderByAxisAngle {
        self._z = val;
        self
    }
    pub fn angle(mut self, val: f64) -> QuaternionBuilderByAxisAngle {
        self._rot = val;
        self
    }
    pub fn normalized(mut self, val: bool) -> QuaternionBuilderByAxisAngle {
        self._normalized = val;
        self
    }

    pub fn build(&self) -> Quaternion {
        let temp_w = (self._rot/2.0).cos();
        let temp_x = self._x * (self._rot/2.0).sin();
        let temp_y = self._y * (self._rot/2.0).sin();
        let temp_z = self._z * (self._rot/2.0).sin();

        let norm = 
            (
                temp_w.powi(2) + 
                temp_x.powi(2) + 
                temp_y.powi(2) + 
                temp_z.powi(2) 
            ).sqrt();

        if self._normalized {
            Quaternion {
                _w: temp_w / norm,
                _x: temp_x / norm,
                _y: temp_y / norm, 
                _z: temp_z / norm,
                _norm: norm
            }
        } else {
            Quaternion {
                _w: temp_w,
                _x: temp_x,
                _y: temp_y, 
                _z: temp_z,
                _norm: norm
            }
        }
    }
}

pub struct QuaternionBuilderByWXYZ {
    _w: f64,
    _x: f64,
    _y: f64,
    _z: f64,
    _normalized: bool,
}

impl QuaternionBuilderByWXYZ {

    pub fn w(mut self, val: f64) -> QuaternionBuilderByWXYZ {
        self._w = val;
        self
    }
    pub fn x(mut self, val: f64) -> QuaternionBuilderByWXYZ {
        self._x = val;
        self
    }
    pub fn y(mut self, val: f64) -> QuaternionBuilderByWXYZ {
        self._y = val;
        self
    }
    pub fn z(mut self, val: f64) -> QuaternionBuilderByWXYZ {
        self._z = val;
        self
    }
    pub fn normalized(mut self, val: bool) -> QuaternionBuilderByWXYZ {
        self._normalized = val;
        self
    }

    pub fn build(&self) -> Quaternion {
        let norm = 
            (
                self._w.powi(2) + 
                self._x.powi(2) + 
                self._y.powi(2) + 
                self._z.powi(2) 
            ).sqrt();

        if self._normalized {
            Quaternion {
                _w: self._w / norm,
                _x: self._x / norm,
                _y: self._y / norm,
                _z: self._z / norm,
                _norm: norm
            }
        } else {
            Quaternion {
                _w: self._w,
                _x: self._x,
                _y: self._y,
                _z: self._z,
                _norm: norm
            }
        }
    }
}

impl Quaternion {
    pub fn init_by_axis_angle() -> QuaternionBuilderByAxisAngle {
        QuaternionBuilderByAxisAngle {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _rot: 0.0,
            _normalized: false,
        }
    }
    pub fn init_by_wxyz() -> QuaternionBuilderByWXYZ {
        QuaternionBuilderByWXYZ {
            _w: 0.0,
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _normalized: false,
        }
    }
    pub fn as_vec3(&self) -> Vec3 {
        Vec3::init()
        .x(self._x)
        .y(self._y)
        .z(self._z)
        .build()
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::init_by_wxyz()
        .w(self._w)
        .x(-1.0 * self._x)
        .y(-1.0 * self._y)
        .z(-1.0 * self._z)
        .build()
    }
    pub fn inverse(&self) -> Quaternion {
        let conj = self.conjugate();
        Quaternion{
            _w: conj._w / self._norm.powi(2),
            _x: conj._x / self._norm.powi(2),
            _y: conj._y / self._norm.powi(2),
            _z: conj._z / self._norm.powi(2),
            _norm: self._norm,
        }
    }
    pub fn normalize(&self) -> Quaternion {
        Quaternion {
            _w: self._w / self._norm,
            _x: self._x / self._norm,
            _y: self._y / self._norm,
            _z: self._z / self._norm,
            _norm: self._norm,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion::init_by_wxyz()
        .w(self._w * other._w - self._x * other._x - self._y * other._y - self._z * other._z)
        .x(self._w * other._x + self._x * other._w + self._y * other._z - self._z * other._y)
        .y(self._w * other._y - self._x * other._z + self._y * other._w + self._z * other._x)
        .z(self._w * other._z + self._x * other._y - self._y * other._x + self._z * other._w)
        .normalized(false)
        .build()
    }
}

// impl Copy for Quaternion { }
// impl Clone for Quaternion {
//     fn clone(&self) -> Quaternion {
//         *self
//     }
// }