use std::borrow::Cow;
use vertex::Attribute;

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AttributeType {
    I8,
    I8I8,
    I8I8I8,
    I8I8I8I8,
    U8,
    U8U8,
    U8U8U8,
    U8U8U8U8,
    I16,
    I16I16,
    I16I16I16,
    I16I16I16I16,
    U16,
    U16U16,
    U16U16U16,
    U16U16U16U16,
    I32,
    I32I32,
    I32I32I32,
    I32I32I32I32,
    U32,
    U32U32,
    U32U32U32,
    U32U32U32U32,
    F32,
    F32F32,
    F32F32F32,
    F32F32F32F32,
    /// 2x2 matrix of `f32`s
    F32x2x2,
    /// 2x3 matrix of `f32`s
    F32x2x3,
    /// 2x3 matrix of `f32`s
    F32x2x4,
    /// 3x2 matrix of `f32`s
    F32x3x2,
    /// 3x3 matrix of `f32`s
    F32x3x3,
    /// 3x4 matrix of `f32`s
    F32x3x4,
    /// 4x2 matrix of `f32`s
    F32x4x2,
    /// 4x3 matrix of `f32`s
    F32x4x3,
    /// 4x4 matrix of `f32`s
    F32x4x4,
    /// Warning: using `f64`s can be very slow.
    F64,
    /// Warning: using `f64`s can be very slow.
    F64F64,
    /// Warning: using `f64`s can be very slow.
    F64F64F64,
    /// Warning: using `f64`s can be very slow.
    F64F64F64F64,
    /// 2x2 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x2x2,
    /// 2x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x2x3,
    /// 2x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x2x4,
    /// 3x2 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x3x2,
    /// 3x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x3x3,
    /// 3x4 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x3x4,
    /// 4x2 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x4x2,
    /// 4x3 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x4x3,
    /// 4x4 matrix of `f64`s
    /// Warning: using `f64`s can be very slow.
    F64x4x4,
}

impl AttributeType {
    /// Returns the number of values for this type.
    pub fn get_num_components(&self) -> usize {
        match *self {
            AttributeType::I8 => 1,
            AttributeType::I8I8 => 2,
            AttributeType::I8I8I8 => 3,
            AttributeType::I8I8I8I8 => 4,
            AttributeType::U8 => 1,
            AttributeType::U8U8 => 2,
            AttributeType::U8U8U8 => 3,
            AttributeType::U8U8U8U8 => 4,
            AttributeType::I16 => 1,
            AttributeType::I16I16 => 2,
            AttributeType::I16I16I16 => 3,
            AttributeType::I16I16I16I16 => 4,
            AttributeType::U16 => 1,
            AttributeType::U16U16 => 2,
            AttributeType::U16U16U16 => 3,
            AttributeType::U16U16U16U16 => 4,
            AttributeType::I32 => 1,
            AttributeType::I32I32 => 2,
            AttributeType::I32I32I32 => 3,
            AttributeType::I32I32I32I32 => 4,
            AttributeType::U32 => 1,
            AttributeType::U32U32 => 2,
            AttributeType::U32U32U32 => 3,
            AttributeType::U32U32U32U32 => 4,
            AttributeType::F32 => 1,
            AttributeType::F32F32 => 2,
            AttributeType::F32F32F32 => 3,
            AttributeType::F32F32F32F32 => 4,
            AttributeType::F32x2x2 => 4,
            AttributeType::F32x2x3 => 6,
            AttributeType::F32x2x4 => 8,
            AttributeType::F32x3x2 => 6,
            AttributeType::F32x3x3 => 9,
            AttributeType::F32x3x4 => 12,
            AttributeType::F32x4x2 => 8,
            AttributeType::F32x4x3 => 12,
            AttributeType::F32x4x4 => 16,
            AttributeType::F64 => 1,
            AttributeType::F64F64 => 2,
            AttributeType::F64F64F64 => 3,
            AttributeType::F64F64F64F64 => 4,
            AttributeType::F64x2x2 => 4,
            AttributeType::F64x2x3 => 6,
            AttributeType::F64x2x4 => 8,
            AttributeType::F64x3x2 => 6,
            AttributeType::F64x3x3 => 9,
            AttributeType::F64x3x4 => 12,
            AttributeType::F64x4x2 => 8,
            AttributeType::F64x4x3 => 12,
            AttributeType::F64x4x4 => 16,
        }
    }
}

/// Describes the layout of each vertex in a vertex buffer.
///
/// The first element is the name of the binding, the second element is the offset
/// from the start of each vertex to this element, and the third element is the type.
pub type VertexFormat = Vec<(Cow<'static, str>, usize, AttributeType)>;

unsafe impl Attribute for i8 {
    fn get_type() -> AttributeType {
        AttributeType::I8
    }
}

unsafe impl Attribute for (i8, i8) {
    fn get_type() -> AttributeType {
        AttributeType::I8I8
    }
}

unsafe impl Attribute for [i8; 2] {
    fn get_type() -> AttributeType {
        AttributeType::I8I8
    }
}

unsafe impl Attribute for (i8, i8, i8) {
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8
    }
}

unsafe impl Attribute for [i8; 3] {
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8
    }
}

unsafe impl Attribute for (i8, i8, i8, i8) {
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8I8
    }
}

unsafe impl Attribute for [i8; 4] {
    fn get_type() -> AttributeType {
        AttributeType::I8I8I8I8
    }
}

unsafe impl Attribute for u8 {
    fn get_type() -> AttributeType {
        AttributeType::U8
    }
}

unsafe impl Attribute for (u8, u8) {
    fn get_type() -> AttributeType {
        AttributeType::U8U8
    }
}

unsafe impl Attribute for [u8; 2] {
    fn get_type() -> AttributeType {
        AttributeType::U8U8
    }
}

unsafe impl Attribute for (u8, u8, u8) {
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8
    }
}

unsafe impl Attribute for [u8; 3] {
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8
    }
}

unsafe impl Attribute for (u8, u8, u8, u8) {
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8U8
    }
}

unsafe impl Attribute for [u8; 4] {
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8U8
    }
}

unsafe impl Attribute for i16 {
    fn get_type() -> AttributeType {
        AttributeType::I16
    }
}

unsafe impl Attribute for (i16, i16) {
    fn get_type() -> AttributeType {
        AttributeType::I16I16
    }
}

unsafe impl Attribute for [i16; 2] {
    fn get_type() -> AttributeType {
        AttributeType::I16I16
    }
}

unsafe impl Attribute for (i16, i16, i16) {
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16
    }
}

unsafe impl Attribute for [i16; 3] {
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16
    }
}

unsafe impl Attribute for (i16, i16, i16, i16) {
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16I16
    }
}

unsafe impl Attribute for [i16; 4] {
    fn get_type() -> AttributeType {
        AttributeType::I16I16I16I16
    }
}

unsafe impl Attribute for u16 {
    fn get_type() -> AttributeType {
        AttributeType::U16
    }
}

unsafe impl Attribute for (u16, u16) {
    fn get_type() -> AttributeType {
        AttributeType::U16U16
    }
}

unsafe impl Attribute for [u16; 2] {
    fn get_type() -> AttributeType {
        AttributeType::U16U16
    }
}

unsafe impl Attribute for (u16, u16, u16) {
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16
    }
}

unsafe impl Attribute for [u16; 3] {
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16
    }
}

unsafe impl Attribute for (u16, u16, u16, u16) {
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16U16
    }
}

unsafe impl Attribute for [u16; 4] {
    fn get_type() -> AttributeType {
        AttributeType::U16U16U16U16
    }
}

unsafe impl Attribute for i32 {
    fn get_type() -> AttributeType {
        AttributeType::I32
    }
}

unsafe impl Attribute for (i32, i32) {
    fn get_type() -> AttributeType {
        AttributeType::I32I32
    }
}

unsafe impl Attribute for [i32; 2] {
    fn get_type() -> AttributeType {
        AttributeType::I32I32
    }
}

unsafe impl Attribute for (i32, i32, i32) {
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32
    }
}

unsafe impl Attribute for [i32; 3] {
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32
    }
}

unsafe impl Attribute for (i32, i32, i32, i32) {
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32I32
    }
}

unsafe impl Attribute for [i32; 4] {
    fn get_type() -> AttributeType {
        AttributeType::I32I32I32I32
    }
}

unsafe impl Attribute for u32 {
    fn get_type() -> AttributeType {
        AttributeType::U32
    }
}

unsafe impl Attribute for (u32, u32) {
    fn get_type() -> AttributeType {
        AttributeType::U32U32
    }
}

unsafe impl Attribute for [u32; 2] {
    fn get_type() -> AttributeType {
        AttributeType::U32U32
    }
}

unsafe impl Attribute for (u32, u32, u32) {
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32
    }
}

unsafe impl Attribute for [u32; 3] {
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32
    }
}

unsafe impl Attribute for (u32, u32, u32, u32) {
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32U32
    }
}

unsafe impl Attribute for [u32; 4] {
    fn get_type() -> AttributeType {
        AttributeType::U32U32U32U32
    }
}

unsafe impl Attribute for f32 {
    fn get_type() -> AttributeType {
        AttributeType::F32
    }
}

unsafe impl Attribute for (f32, f32) {
    fn get_type() -> AttributeType {
        AttributeType::F32F32
    }
}

unsafe impl Attribute for [f32; 2] {
    fn get_type() -> AttributeType {
        AttributeType::F32F32
    }
}

unsafe impl Attribute for (f32, f32, f32) {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32
    }
}

unsafe impl Attribute for [f32; 3] {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32
    }
}

unsafe impl Attribute for (f32, f32, f32, f32) {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }
}

unsafe impl Attribute for [f32; 4] {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }
}

unsafe impl Attribute for [[f32; 2]; 2] {
    fn get_type() -> AttributeType {
        AttributeType::F32x2x2
    }
}

unsafe impl Attribute for [[f32; 3]; 3] {
    fn get_type() -> AttributeType {
        AttributeType::F32x3x3
    }
}

unsafe impl Attribute for [[f32; 4]; 4] {
    fn get_type() -> AttributeType {
        AttributeType::F32x4x4
    }
}

unsafe impl Attribute for f64 {
    fn get_type() -> AttributeType {
        AttributeType::F64
    }
}

unsafe impl Attribute for (f64, f64) {
    fn get_type() -> AttributeType {
        AttributeType::F64F64
    }
}

unsafe impl Attribute for [f64; 2] {
    fn get_type() -> AttributeType {
        AttributeType::F64F64
    }
}

unsafe impl Attribute for (f64, f64, f64) {
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64
    }
}

unsafe impl Attribute for [f64; 3] {
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64
    }
}

unsafe impl Attribute for (f64, f64, f64, f64) {
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64F64
    }
}

unsafe impl Attribute for [f64; 4] {
    fn get_type() -> AttributeType {
        AttributeType::F64F64F64F64
    }
}

unsafe impl Attribute for [[f64; 2]; 2] {
    fn get_type() -> AttributeType {
        AttributeType::F64x2x2
    }
}

unsafe impl Attribute for [[f64; 3]; 3] {
    fn get_type() -> AttributeType {
        AttributeType::F64x3x3
    }
}

unsafe impl Attribute for [[f64; 4]; 4] {
    fn get_type() -> AttributeType {
        AttributeType::F64x4x4
    }
}

macro_rules! implement_attribute {
    ( $impl_ty:ty => $attrib_ty:path ) => {
        unsafe impl Attribute for $impl_ty {
            fn get_type() -> AttributeType {
                $attrib_ty
            }
        }
    }
}

#[cfg(feature = "nalgebra")]
pub mod nalgebra_attr_impl {
    use vertex::Attribute;
    use super::AttributeType;

    use nalgebra::{Pnt1, Pnt2, Pnt3};
    use nalgebra::{Vec1, Vec2, Vec3, Vec4};
    use nalgebra::{Mat1, Mat2, Mat3, Mat4};
    use nalgebra::{Quat, Rot2, Rot3, Rot4};

    implement_attribute!(Pnt1<i8> => AttributeType::I8);
    implement_attribute!(Pnt2<i8> => AttributeType::I8I8);
    implement_attribute!(Pnt3<i8> => AttributeType::I8I8I8);

    implement_attribute!(Pnt1<u8> => AttributeType::U8);
    implement_attribute!(Pnt2<u8> => AttributeType::U8U8);
    implement_attribute!(Pnt3<u8> => AttributeType::U8U8U8);

    implement_attribute!(Pnt1<i16> => AttributeType::I16);
    implement_attribute!(Pnt2<i16> => AttributeType::I16I16);
    implement_attribute!(Pnt3<i16> => AttributeType::I16I16I16);

    implement_attribute!(Pnt1<u16> => AttributeType::U16);
    implement_attribute!(Pnt2<u16> => AttributeType::U16U16);
    implement_attribute!(Pnt3<u16> => AttributeType::U16U16U16);

    implement_attribute!(Pnt1<i32> => AttributeType::I32);
    implement_attribute!(Pnt2<i32> => AttributeType::I32I32);
    implement_attribute!(Pnt3<i32> => AttributeType::I32I32I32);

    implement_attribute!(Pnt1<u32> => AttributeType::U32);
    implement_attribute!(Pnt2<u32> => AttributeType::U32U32);
    implement_attribute!(Pnt3<u32> => AttributeType::U32U32U32);

    implement_attribute!(Pnt1<f32> => AttributeType::F32);
    implement_attribute!(Pnt2<f32> => AttributeType::F32F32);
    implement_attribute!(Pnt3<f32> => AttributeType::F32F32F32);

    implement_attribute!(Pnt1<f64> => AttributeType::F64);
    implement_attribute!(Pnt2<f64> => AttributeType::F64F64);
    implement_attribute!(Pnt3<f64> => AttributeType::F64F64F64);

    implement_attribute!(Vec1<i8> => AttributeType::I8);
    implement_attribute!(Vec2<i8> => AttributeType::I8I8);
    implement_attribute!(Vec3<i8> => AttributeType::I8I8I8);
    implement_attribute!(Vec4<i8> => AttributeType::I8I8I8I8);

    implement_attribute!(Vec1<u8> => AttributeType::U8);
    implement_attribute!(Vec2<u8> => AttributeType::U8U8);
    implement_attribute!(Vec3<u8> => AttributeType::U8U8U8);
    implement_attribute!(Vec4<u8> => AttributeType::U8U8U8U8);

    implement_attribute!(Vec1<i16> => AttributeType::I16);
    implement_attribute!(Vec2<i16> => AttributeType::I16I16);
    implement_attribute!(Vec3<i16> => AttributeType::I16I16I16);
    implement_attribute!(Vec4<i16> => AttributeType::I16I16I16I16);

    implement_attribute!(Vec1<u16> => AttributeType::U16);
    implement_attribute!(Vec2<u16> => AttributeType::U16U16);
    implement_attribute!(Vec3<u16> => AttributeType::U16U16U16);
    implement_attribute!(Vec4<u16> => AttributeType::U16U16U16U16);

    implement_attribute!(Vec1<i32> => AttributeType::I32);
    implement_attribute!(Vec2<i32> => AttributeType::I32I32);
    implement_attribute!(Vec3<i32> => AttributeType::I32I32I32);
    implement_attribute!(Vec4<i32> => AttributeType::I32I32I32I32);

    implement_attribute!(Vec1<u32> => AttributeType::U32);
    implement_attribute!(Vec2<u32> => AttributeType::U32U32);
    implement_attribute!(Vec3<u32> => AttributeType::U32U32U32);
    implement_attribute!(Vec4<u32> => AttributeType::U32U32U32U32);

    implement_attribute!(Vec1<f32> => AttributeType::F32);
    implement_attribute!(Vec2<f32> => AttributeType::F32F32);
    implement_attribute!(Vec3<f32> => AttributeType::F32F32F32);
    implement_attribute!(Vec4<f32> => AttributeType::F32F32F32F32);

    implement_attribute!(Vec1<f64> => AttributeType::F64);
    implement_attribute!(Vec2<f64> => AttributeType::F64F64);
    implement_attribute!(Vec3<f64> => AttributeType::F64F64F64);
    implement_attribute!(Vec4<f64> => AttributeType::F64F64F64F64);

    implement_attribute!(Mat1<f32> => AttributeType::F32);
    implement_attribute!(Mat2<f32> => AttributeType::F32x2x2);
    implement_attribute!(Mat3<f32> => AttributeType::F32x3x3);
    implement_attribute!(Mat4<f32> => AttributeType::F32x4x4);

    implement_attribute!(Mat1<f64> => AttributeType::F64);
    implement_attribute!(Mat2<f64> => AttributeType::F64x2x2);
    implement_attribute!(Mat3<f64> => AttributeType::F64x3x3);
    implement_attribute!(Mat4<f64> => AttributeType::F64x4x4);

    implement_attribute!(Quat<i8>  => AttributeType::I8I8I8I8);
    implement_attribute!(Quat<u8>  => AttributeType::U8U8U8U8);
    implement_attribute!(Quat<i16> => AttributeType::I16I16I16I16);
    implement_attribute!(Quat<u16> => AttributeType::U16U16U16U16);
    implement_attribute!(Quat<i32> => AttributeType::I32I32I32I32);
    implement_attribute!(Quat<u32> => AttributeType::U32U32U32U32);
    implement_attribute!(Quat<f32> => AttributeType::F32F32F32F32);
    implement_attribute!(Quat<f64> => AttributeType::F64F64F64F64);

    implement_attribute!(Rot2<f32> => AttributeType::F32x2x2);
    implement_attribute!(Rot3<f32> => AttributeType::F32x3x3);
    implement_attribute!(Rot4<f32> => AttributeType::F32x4x4);

    implement_attribute!(Rot2<f64> => AttributeType::F64x2x2);
    implement_attribute!(Rot3<f64> => AttributeType::F64x3x3);
    implement_attribute!(Rot4<f64> => AttributeType::F64x4x4);
}