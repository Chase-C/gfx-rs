// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Render target specification.

use std::fmt;

// TODO: Really tighten up the terminology here.

/// A depth value, specifying which plane to select out of a 3D texture.
pub type Layer = u16;
/// Mipmap level to select in a texture.
pub type Level = u8;
/// A single depth value from a depth buffer.
pub type Depth = f32;
/// A single value from a stencil stencstencil buffer.
pub type Stencil = u8;

/// A screen space rectangle
#[allow(missing_doc)]
#[deriving(Clone, PartialEq, Show)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

/// A color with floating-point components.
pub type Color = [f32, ..4];

/// How to clear a frame.
pub struct ClearData {
    /// If set, the color buffer of the frame will be cleared to this.
    pub color: Option<Color>,
    /// If set, the depth buffer of the frame will be cleared to this.
    pub depth: Option<Depth>,
    /// If set, the stencil buffer of the frame will be cleared to this.
    pub stencil: Option<Stencil>,
}

impl Clone for ClearData {
    fn clone(&self) -> ClearData {
        ClearData {
            color: self.color,
            depth: self.depth,
            stencil: self.stencil,
        }
    }
}

impl fmt::Show for ClearData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "ClearData {{ color: "));
        match self.color {
            Some(ref c) => try!(write!(f, "Some({})", c.as_slice())),
            None => try!(write!(f, "None")),
        }
        write!(f, ", depth: {}, stencil: {} }}", self.depth, self.stencil)
    }
}

/// When rendering, each "output" of the fragment shader goes to a specific target. A `Plane` can
/// be bound to a target, causing writes to that target to affect the `Plane`.
#[deriving(Clone, Show)]
pub enum Target {
    /// Color data.
    ///
    /// # Portability Note
    ///
    /// The device is only required to expose one color target.
    TargetColor(u8),
    /// Depth data.
    TargetDepth,
    /// Stencil data.
    TargetStencil,
    /// A target for both depth and stencil data at once.
    TargetDepthStencil,
}
