use bevy::prelude::*;

pub struct Hex {
    q: i32,
    r: i32,
}

struct Cube<T> {
    q: T,
    r: T,
    s: T,
}

const SIZE: f32 = 1.0;

impl Hex {
    ///We use x and z in this method because bevy uses right handed y-up coordinate system
    pub fn from_world(position: Vec3) -> Self {
        let q = (3_f32.sqrt()/3. * position.x - 1./3. * position.z) / SIZE;
        let r = (2./3. * position.z ) / SIZE;
        Hex::axial_round(q, r)
    }

    fn axial_round(q: f32, r: f32) -> Self {
        let cube = Hex::axial_to_cube(q, r);
        let cube_round = Hex::cube_round(cube);
        Hex::cube_to_axial(cube_round)
    }

    fn axial_to_cube(q: f32, r: f32) -> Cube<f32> {
        Cube {
            q,
            r,
            s: -q-r,
        }
    }

    fn cube_to_axial(cube: Cube<i32>) -> Self {
        Self {
            q: cube.q,
            r: cube.r,
        }
    }

    fn cube_round(cube: Cube<f32>) -> Cube<i32> {
        let mut q = cube.q.round();
        let mut r = cube.r.round();
        let mut s = cube.s.round();

        let q_diff = (q - cube.q).abs();
        let r_diff = (r - cube.r).abs();
        let s_diff = (s - cube.s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r-s;
        }
        else if r_diff > s_diff {
            r = -q-s;
        }
        else {
            s = -q-r;
        }

        Cube {
            q: q as i32,
            r: r as i32,
            s: s as i32,
        }
    }

    pub fn to_world(&self) -> Vec3 {
        let q = self.q as f32;
        let r = self.r as f32;
        let x = SIZE * (3_f32.sqrt() * q + 3_f32.sqrt() / 2_f32 * r);
        let z = SIZE * (3./2. * r);
        Vec3::new(x, 0.0, z)
    }
}