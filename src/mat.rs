#![allow(non_camel_case_types)]

use crate::vec::{vec2, vec3, vec4};

pub type mat2x2<T> = [vec2<T>; 2];
pub type mat3x2<T> = [vec2<T>; 3];
pub type mat4x2<T> = [vec2<T>; 4];

pub type mat2x3<T> = [vec3<T>; 2];
pub type mat3x3<T> = [vec3<T>; 3];
pub type mat4x3<T> = [vec3<T>; 4];

pub type mat2x4<T> = [vec4<T>; 2];
pub type mat3x4<T> = [vec4<T>; 3];
pub type mat4x4<T> = [vec4<T>; 4];
