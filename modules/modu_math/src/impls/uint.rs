use crate::{Scalar, ScalarBackend};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    x: T,
    y: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    x: T,
    y: T,
    z: T,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec4 {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl Scalar for T {}

impl ScalarBackend<2> for T {
    type Vec = Vec2;

    #[inline(always)]
    fn vec_eq(vec: &Self::Vec, other: &Self::Vec) -> bool {
        vec.x == other.x && vec.y == other.y
    }

    #[inline(always)]
    fn vec_ne(vec: &Self::Vec, other: &Self::Vec) -> bool {
        vec.x != other.x || vec.y != other.y
    }

    #[inline(always)]
    fn vec_neg(_: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_not(vec: Self::Vec) -> Self::Vec {
        Vec2 {
            x: !vec.x,
            y: !vec.y,
        }
    }

    #[inline(always)]
    fn vec_add(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x + rhs.x,
            y: vec.y + rhs.y,
        }
    }

    #[inline(always)]
    fn vec_sub(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x - rhs.x,
            y: vec.y - rhs.y,
        }
    }

    #[inline(always)]
    fn vec_mul(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x * rhs.x,
            y: vec.y * rhs.y,
        }
    }

    #[inline(always)]
    fn vec_div(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x / rhs.x,
            y: vec.y / rhs.y,
        }
    }

    #[inline(always)]
    fn vec_rem(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x % rhs.x,
            y: vec.y % rhs.y,
        }
    }

    #[inline(always)]
    fn vec_shl(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x << rhs.x,
            y: vec.y << rhs.y,
        }
    }

    #[inline(always)]
    fn vec_shr(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x >> rhs.x,
            y: vec.y >> rhs.y,
        }
    }

    #[inline(always)]
    fn vec_bitand(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x & rhs.x,
            y: vec.y & rhs.y,
        }
    }

    #[inline(always)]
    fn vec_bitor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x | rhs.x,
            y: vec.y | rhs.y,
        }
    }

    #[inline(always)]
    fn vec_bitxor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x ^ rhs.x,
            y: vec.y ^ rhs.y,
        }
    }
}

impl ScalarBackend<3> for T {
    type Vec = Vec3;

    #[inline(always)]
    fn vec_eq(vec: &Self::Vec, other: &Self::Vec) -> bool {
        vec.x == other.x && vec.y == other.y && vec.z == other.z
    }

    #[inline(always)]
    fn vec_ne(vec: &Self::Vec, other: &Self::Vec) -> bool {
        vec.x != other.x || vec.y != other.y || vec.z != other.z
    }

    #[inline(always)]
    fn vec_neg(_: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_not(vec: Self::Vec) -> Self::Vec {
        Vec3 {
            x: !vec.x,
            y: !vec.y,
            z: !vec.z,
        }
    }

    #[inline(always)]
    fn vec_add(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x + rhs.x,
            y: vec.y + rhs.y,
            z: vec.z + rhs.z,
        }
    }

    #[inline(always)]
    fn vec_sub(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x - rhs.x,
            y: vec.y - rhs.y,
            z: vec.z - rhs.z,
        }
    }

    #[inline(always)]
    fn vec_mul(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x * rhs.x,
            y: vec.y * rhs.y,
            z: vec.z * rhs.z,
        }
    }

    #[inline(always)]
    fn vec_div(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x / rhs.x,
            y: vec.y / rhs.y,
            z: vec.z / rhs.z,
        }
    }

    #[inline(always)]
    fn vec_rem(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x % rhs.x,
            y: vec.y % rhs.y,
            z: vec.z % rhs.z,
        }
    }

    #[inline(always)]
    fn vec_shl(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x << rhs.x,
            y: vec.y << rhs.y,
            z: vec.z << rhs.z,
        }
    }

    #[inline(always)]
    fn vec_shr(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x >> rhs.x,
            y: vec.y >> rhs.y,
            z: vec.z >> rhs.z,
        }
    }

    #[inline(always)]
    fn vec_bitand(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x & rhs.x,
            y: vec.y & rhs.y,
            z: vec.z & rhs.z,
        }
    }

    #[inline(always)]
    fn vec_bitor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x | rhs.x,
            y: vec.y | rhs.y,
            z: vec.z | rhs.z,
        }
    }

    #[inline(always)]
    fn vec_bitxor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x ^ rhs.x,
            y: vec.y ^ rhs.y,
            z: vec.z ^ rhs.z,
        }
    }
}

impl ScalarBackend<4> for T {
    type Vec = Vec4;

    #[inline(always)]
    fn vec_eq(vec: &Self::Vec, other: &Self::Vec) -> bool {
        vec.x == other.x && vec.y == other.y && vec.z == other.z && vec.w == other.w
    }

    #[inline(always)]
    fn vec_ne(vec: &Self::Vec, other: &Self::Vec) -> bool {
        vec.x != other.x || vec.y != other.y || vec.z != other.z || vec.w != other.w
    }

    #[inline(always)]
    fn vec_neg(_: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_not(vec: Self::Vec) -> Self::Vec {
        Vec4 {
            x: !vec.x,
            y: !vec.y,
            z: !vec.z,
            w: !vec.w,
        }
    }

    #[inline(always)]
    fn vec_add(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x + rhs.x,
            y: vec.y + rhs.y,
            z: vec.z + rhs.z,
            w: vec.w + rhs.w,
        }
    }

    #[inline(always)]
    fn vec_sub(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x - rhs.x,
            y: vec.y - rhs.y,
            z: vec.z - rhs.z,
            w: vec.w - rhs.w,
        }
    }

    #[inline(always)]
    fn vec_mul(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x * rhs.x,
            y: vec.y * rhs.y,
            z: vec.z * rhs.z,
            w: vec.w * rhs.w,
        }
    }

    #[inline(always)]
    fn vec_div(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x / rhs.x,
            y: vec.y / rhs.y,
            z: vec.z / rhs.z,
            w: vec.w / rhs.w,
        }
    }

    #[inline(always)]
    fn vec_rem(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x % rhs.x,
            y: vec.y % rhs.y,
            z: vec.z % rhs.z,
            w: vec.w % rhs.w,
        }
    }

    #[inline(always)]
    fn vec_shl(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x << rhs.x,
            y: vec.y << rhs.y,
            z: vec.z << rhs.z,
            w: vec.w << rhs.w,
        }
    }

    #[inline(always)]
    fn vec_shr(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x >> rhs.x,
            y: vec.y >> rhs.y,
            z: vec.z >> rhs.z,
            w: vec.w >> rhs.w,
        }
    }

    #[inline(always)]
    fn vec_bitand(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x & rhs.x,
            y: vec.y & rhs.y,
            z: vec.z & rhs.z,
            w: vec.w & rhs.w,
        }
    }

    #[inline(always)]
    fn vec_bitor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x | rhs.x,
            y: vec.y | rhs.y,
            z: vec.z | rhs.z,
            w: vec.w | rhs.w,
        }
    }

    #[inline(always)]
    fn vec_bitxor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x ^ rhs.x,
            y: vec.y ^ rhs.y,
            z: vec.z ^ rhs.z,
            w: vec.w ^ rhs.w,
        }
    }
}
