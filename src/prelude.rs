//! A collection of commonly used items that we recommend importing for ease of use.

pub use app::{self, App, LoopMode};
pub use audio;
pub use color::{Hsl, Hsla, Hsv, Hsva, Rgb, Rgba};
pub use color::named::*;
pub use event::SimpleWindowEvent::*;
pub use event::{Event, Key};
pub use frame::Frame;
pub use geom::{self, Rect, Cuboid};
pub use io::{load_from_json, load_from_toml, safe_file_save, save_to_json, save_to_toml};
pub use math::{clamp, deg_to_rad, rad_to_deg, turns_to_rad, rad_to_turns, fmod, map_range,
               partial_min, partial_max, pt1, pt2, pt3, vec1, vec2, vec3, vec4, Point2, Point3,
               Vector2, Vector3};
pub use math::prelude::*;
pub use math::num_traits::*;
pub use osc;
pub use rand::{random, random_f32, random_f64, random_range};
pub use ui;
pub use window::{self, Id as WindowId};

// The following constants have "regular" names for the `DefaultScalar` type and type suffixes for
// other types. If the `DefaultScalar` changes, these should probably change too.

pub use std::f32::consts::PI as PI;
pub use std::f64::consts::PI as PI_F64;

/// Two times PI.
pub const TAU: f32 = PI * 2.0;
/// Two times PI.
pub const TAU_F64: f64 = PI_F64 * 2.0;
