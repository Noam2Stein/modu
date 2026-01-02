use crate::{FloatsBackend, Scalar, ScalarBackend};

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
    fn vec_neg(vec: Self::Vec) -> Self::Vec {
        Vec2 {
            x: -vec.x,
            y: -vec.y,
        }
    }

    #[inline(always)]
    fn vec_not(_: Self::Vec) -> Self::Vec {
        unreachable!()
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
    fn vec_shl(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_shr(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitand(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitor(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitxor(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
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
    fn vec_neg(vec: Self::Vec) -> Self::Vec {
        Vec3 {
            x: -vec.x,
            y: -vec.y,
            z: -vec.z,
        }
    }

    #[inline(always)]
    fn vec_not(_: Self::Vec) -> Self::Vec {
        unreachable!()
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
    fn vec_shl(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_shr(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitand(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitor(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitxor(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
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
    fn vec_neg(vec: Self::Vec) -> Self::Vec {
        Vec4 {
            x: -vec.x,
            y: -vec.y,
            z: -vec.z,
            w: -vec.w,
        }
    }

    #[inline(always)]
    fn vec_not(_: Self::Vec) -> Self::Vec {
        unreachable!()
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
    fn vec_shl(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_shr(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitand(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitor(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }

    #[inline(always)]
    fn vec_bitxor(_: Self::Vec, _: Self::Vec) -> Self::Vec {
        unreachable!()
    }
}

impl FloatsBackend<2> for T {
    #[inline(always)]
    fn vec_abs(vec: Self::Vec) -> Self::Vec {
        Vec2 {
            x: vec.x.abs(),
            y: vec.y.abs(),
        }
    }
}

impl FloatsBackend<3> for T {
    #[inline(always)]
    fn vec_abs(vec: Self::Vec) -> Self::Vec {
        Vec3 {
            x: vec.x.abs(),
            y: vec.y.abs(),
            z: vec.z.abs(),
        }
    }
}

impl FloatsBackend<4> for T {
    #[inline(always)]
    fn vec_abs(vec: Self::Vec) -> Self::Vec {
        Vec4 {
            x: vec.x.abs(),
            y: vec.y.abs(),
            z: vec.z.abs(),
            w: vec.w.abs(),
        }
    }
}
