#![allow(non_camel_case_types)]

use serde::ser::SerializeTupleStruct;
use serde::{Serialize, Serializer};

use crate::f16;

#[derive(Copy, Clone, Debug, Default)]
pub struct vec2<T>(pub T, pub T);

impl Serialize for vec2<f16> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec2@f16", 2)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.end()
    }
}

impl Serialize for vec2<i32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec2@i32", 2)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.end()
    }
}

impl Serialize for vec2<u32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec2@u32", 2)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.end()
    }
}

impl Serialize for vec2<f32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec2@f32", 2)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.end()
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct vec3<T>(pub T, pub T, pub T);

impl Serialize for vec3<f16> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec3@f16", 3)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.end()
    }
}

impl Serialize for vec3<i32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec3@i32", 3)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.end()
    }
}

impl Serialize for vec3<u32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec3@u32", 3)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.end()
    }
}

impl Serialize for vec3<f32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec3@f32", 3)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.end()
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct vec4<T>(pub T, pub T, pub T, pub T);

impl Serialize for vec4<f16> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec4@f16", 4)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.serialize_field(&self.3)?;
        s.end()
    }
}

impl Serialize for vec4<i32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec4@i32", 4)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.serialize_field(&self.3)?;
        s.end()
    }
}

impl Serialize for vec4<u32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec4@u32", 4)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.serialize_field(&self.3)?;
        s.end()
    }
}

impl Serialize for vec4<f32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple_struct("vec4@f32", 4)?;
        s.serialize_field(&self.0)?;
        s.serialize_field(&self.1)?;
        s.serialize_field(&self.2)?;
        s.serialize_field(&self.3)?;
        s.end()
    }
}
