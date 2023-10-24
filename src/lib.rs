//!
//! ```
//! # use half::f16;
//! # use serde::Serialize;
//! # use serde_webgpu::vec::vec4;
//! # use serde_webgpu::mat::mat4x4;
//! # use serde_webgpu::serialize_webgpu_buffer;
//!
//! #[derive(Serialize)]
//! struct Uniform {
//!     a: f16,
//!     b: mat4x4<f32>,
//! }
//!
//! let uniform = Uniform {
//!     a: f16::from_f32(123.456),
//!     b: [
//!         vec4([1.0, 2.0, 3.0, 4.0]),
//!         vec4([4.0, 5.0, 7.0, 8.0]),
//!         vec4([1.0, 2.0, 3.0, 4.0]),
//!         vec4([5.0, 6.0, 7.0, 8.0]),
//!     ],
//! };
//!
//! serialize_webgpu_buffer(&uniform).unwrap();
//! ```
//!

use std::cmp::max;
use std::fmt::{Debug, Display, Formatter};

use serde::ser::{Impossible, SerializeSeq, SerializeStruct, SerializeTuple, SerializeTupleStruct};
use serde::{Serialize, Serializer};

pub mod mat;
pub mod vec;

#[allow(non_camel_case_types)]
pub type f16 = half::f16;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub enum Align {
    #[default]
    Align0,
    Align1,
    Align2,
    Align4,
    Align8,
    Align16,
}

impl Align {
    pub fn with(self, rhs: Self) -> Self {
        max(self, rhs)
    }
    pub fn append(&mut self, rhs: Self) {
        *self = max(*self, rhs);
    }
    pub fn value(self) -> usize {
        match self {
            Align::Align0 => 0,
            Align::Align1 => 1,
            Align::Align2 => 2,
            Align::Align4 => 4,
            Align::Align8 => 8,
            Align::Align16 => 16,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum WebGPUItem {
    Align(Align),
    Data(usize),
}

#[derive(Clone, Debug, Default)]
struct WebGPUBlock {
    buffer: Vec<u8>,
    items: Vec<WebGPUItem>,
}

impl WebGPUBlock {
    pub fn append(&mut self, i: &[u8]) {
        self.buffer.extend_from_slice(i);
        self.items.push(WebGPUItem::Data(i.len()));
    }

    pub fn align(&mut self, align: Align) -> usize {
        let index = self.items.len();
        self.items.push(WebGPUItem::Align(align));
        index
    }

    pub fn get_align(&self, index: usize) -> Align {
        match self.items[index] {
            WebGPUItem::Align(a) => a,
            _ => unreachable!(),
        }
    }

    fn align_append(&mut self, index: usize, align: Align) {
        match &mut self.items[index] {
            WebGPUItem::Align(a) => a.append(align),
            _ => unreachable!(),
        }
    }

    fn compute_layout(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut offset = 0;
        for &i in &self.items {
            match i {
                WebGPUItem::Align(Align::Align0) => {}
                WebGPUItem::Align(align) => {
                    let a = align.value() - 1;
                    let p = buffer.len();
                    let s = (p + a) & !a;
                    for _ in 0..(s - p) {
                        buffer.push(0);
                    }
                }
                WebGPUItem::Data(length) => {
                    let next_offset = offset + length;
                    buffer.extend_from_slice(&self.buffer[offset..next_offset]);
                    offset = next_offset;
                }
            }
        }
        assert_eq!(offset, self.buffer.len());
        buffer
    }
}

#[derive(Debug)]
pub struct WebGPUSerializeError {
    msg: String,
}

impl std::error::Error for WebGPUSerializeError {}

impl Display for WebGPUSerializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl serde::ser::Error for WebGPUSerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            msg: format!("{}", msg),
        }
    }
}

struct WebGPUSerializer<'s> {
    write: &'s mut WebGPUBlock,
}

impl<'s> Serializer for WebGPUSerializer<'s> {
    type Ok = Align;
    type Error = WebGPUSerializeError;

    type SerializeSeq = WebGPUSerializeStruct<'s>;
    type SerializeTuple = WebGPUSerializeStruct<'s>;
    type SerializeTupleStruct = WebGPUSerializeStruct<'s>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = WebGPUSerializeStruct<'s>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.write.append(&[v as u8]);
        Ok(Align::Align1)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write.append(&[v as u8]);
        Ok(Align::Align1)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align2);
        self.write.append(&i16::to_le_bytes(v));
        Ok(Align::Align4)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align4);
        self.write.append(&i32::to_le_bytes(v));
        Ok(Align::Align4)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(serde::ser::Error::custom("i64 is not supported"))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write.append(&[v]);
        Ok(Align::Align1)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align4);
        self.write.append(&u16::to_le_bytes(v));
        Ok(Align::Align4)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align4);
        self.write.append(&u32::to_le_bytes(v));
        Ok(Align::Align4)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(serde::ser::Error::custom("u64 is not supported"))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align4);
        self.write.append(&f32::to_le_bytes(v));
        Ok(Align::Align4)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(serde::ser::Error::custom("f64 is not supported"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align4);
        self.write.append(&u32::to_le_bytes(v as u32));
        Ok(Align::Align4)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align1);
        self.write.append(v.as_bytes());
        Ok(Align::Align1)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.write.align(Align::Align1);
        self.write.append(v);
        Ok(Align::Align1)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(serde::ser::Error::custom("enum is not supported"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(serde::ser::Error::custom("enum is not supported"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple(0)?.end()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple_struct(name, 0)?.end()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(serde::ser::Error::custom("enum is not supported"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        match name {
            "f16" => {
                return value.serialize(WebGPUSerializer { write: self.write });
            }
            _ => {}
        }

        let mut s = self.serialize_tuple_struct(name, 1)?;
        s.serialize_element(value)?;
        s.end()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(serde::ser::Error::custom("enum is not supported"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(WebGPUSerializeStruct::new(self.write, Align::Align0))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(WebGPUSerializeStruct::new(self.write, Align::Align0))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let vec_align = match len {
            2 => match name {
                "vec2@f16" => Some(Align::Align4),
                "vec2@i32" | "vec2@u32" | "vec2@f32" => Some(Align::Align8),
                _ => None,
            },
            3 => match name {
                "vec3@f16" => Some(Align::Align8),
                "vec3@i32" | "vec3@u32" | "vec3@f32" => Some(Align::Align16),
                _ => None,
            },
            4 => match name {
                "vec4@f16" => Some(Align::Align8),
                "vec4@i32" | "vec4@u32" | "vec4@f32" => Some(Align::Align16),
                _ => None,
            },
            _ => None,
        };

        if let Some(align) = vec_align {
            return Ok(WebGPUSerializeStruct::new(self.write, align));
        }

        Ok(WebGPUSerializeStruct::new(self.write, Align::Align0))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(serde::ser::Error::custom("enum is not supported"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(serde::ser::Error::custom("map is not supported"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(WebGPUSerializeStruct::new(self.write, Align::Align0))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(serde::ser::Error::custom("enum is not supported"))
    }
}

struct WebGPUSerializeStruct<'s> {
    write: &'s mut WebGPUBlock,
    align_index: usize,
    ext_align: Align,
}

impl<'s> WebGPUSerializeStruct<'s> {
    fn align_to(&mut self, align: Align) {
        self.write.align_append(self.align_index, align);
    }
}

impl<'s> WebGPUSerializeStruct<'s> {
    fn new(write: &'s mut WebGPUBlock, ext_align: Align) -> Self {
        let align_index = write.align(Default::default());
        Self {
            write,
            align_index,
            ext_align,
        }
    }

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), WebGPUSerializeError>
    where
        T: Serialize,
    {
        let align = value.serialize(WebGPUSerializer { write: self.write })?;
        self.align_to(align);
        Ok(())
    }

    fn end(self) -> Result<Align, WebGPUSerializeError> {
        let align = self.write.get_align(self.align_index);
        if align == Align::Align0 {
            return Err(serde::ser::Error::custom("zero size type is not supported"));
        }
        self.write.align(align);
        Ok(align.with(self.ext_align))
    }
}

impl<'s> SerializeTuple for WebGPUSerializeStruct<'s> {
    type Ok = <WebGPUSerializer<'s> as Serializer>::Ok;
    type Error = <WebGPUSerializer<'s> as Serializer>::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

impl<'s> SerializeTupleStruct for WebGPUSerializeStruct<'s> {
    type Ok = <WebGPUSerializer<'s> as Serializer>::Ok;
    type Error = <WebGPUSerializer<'s> as Serializer>::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

impl<'s> SerializeStruct for WebGPUSerializeStruct<'s> {
    type Ok = <WebGPUSerializer<'s> as Serializer>::Ok;
    type Error = <WebGPUSerializer<'s> as Serializer>::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

impl<'s> SerializeSeq for WebGPUSerializeStruct<'s> {
    type Ok = <WebGPUSerializer<'s> as Serializer>::Ok;
    type Error = <WebGPUSerializer<'s> as Serializer>::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

fn serialize_webgpu_base<T: Serialize>(value: &T) -> Result<WebGPUBlock, WebGPUSerializeError> {
    let mut block = WebGPUBlock::default();
    let serializer = WebGPUSerializer { write: &mut block };
    value.serialize(serializer)?;
    Ok(block)
}

pub fn serialize_webgpu<T: Serialize>(value: &T) -> Result<Vec<u8>, WebGPUSerializeError> {
    let block = serialize_webgpu_base(value)?;
    Ok(block.compute_layout())
}

pub fn serialize_webgpu_buffer<T: Serialize>(value: &T) -> Result<Vec<u8>, WebGPUSerializeError> {
    let mut block = serialize_webgpu_base(value)?;
    block.align(Align::Align16);
    Ok(block.compute_layout())
}
