use std::collections::HashMap;
use std::convert::{From, Into};
use std::ffi::c_void;
use std::fmt;

use gl::{types::*, *};

use crate::debug::gl;

pub struct Texture<const T: TextureType> {
    id: GLuint,
}

impl<const T: TextureType> Texture<T> {
    pub fn activate(&self, index: u32) {
        if index >= gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS {
            panic!(
                "Cannot have {} textures - exceeds {} limit",
                index + 1,
                gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS
            );
        }

        gl!(ActiveTexture(gl::TEXTURE0 + index));
        self.bind();

        log::trace!("Activated and bound {} at GL_TEXTURE{}", self, index);
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    fn bind(&self) {
        gl!(BindTexture(Into::into(T), self.id));
    }
}

impl<const T: TextureType> Drop for Texture<T> {
    fn drop(&mut self) {
        log::debug!("Deleting {}", self);

        gl!(DeleteTextures(1, &self.id));
    }
}

impl<const T: TextureType> fmt::Display for Texture<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} texture {}", T, self.id)
    }
}

pub type Texture2D = Texture<{ TextureType::Texture2D }>;
pub type Texture3D = Texture<{ TextureType::Texture3D }>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TextureType {
    Texture2D,
    Texture3D,
}

impl From<TextureType> for GLenum {
    fn from(t: TextureType) -> GLenum {
        match t {
            TextureType::Texture2D => gl::TEXTURE_2D,
            TextureType::Texture3D => gl::TEXTURE_3D,
        }
    }
}

impl fmt::Display for TextureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextureType::Texture2D => write!(f, "2D"),
            TextureType::Texture3D => write!(f, "3D"),
        }
    }
}

pub struct TextureBuilder<'a, I: Image, const T: TextureType> {
    img: &'a I,
    parameters: HashMap<GLenum, Parameter>,
    generate_mipmap: bool,
}

impl<'a, I: Image, const T: TextureType> TextureBuilder<'a, I, T> {
    pub fn new(img: &'a I) -> Self {
        Self {
            img,
            parameters: HashMap::new(),
            generate_mipmap: false,
        }
    }

    pub fn build(self) -> Texture<{ T }> {
        let mut id = 0;

        gl!(GenTextures(1, &mut id));

        let texture = Texture { id };

        texture.bind();

        for (key, parameter) in self.parameters {
            match parameter {
                //Parameter::Float(f) => gl!(TexParameterf(Into::into(T), key, f)),
                Parameter::Int(i) => gl!(TexParameteri(Into::into(T), key, i)),
                Parameter::Floats(fv) => gl!(TexParameterfv(Into::into(T), key, fv.as_ptr())),
            }
        }

        match T {
            TextureType::Texture2D => {
                gl!(TexImage2D(
                    Into::into(T),
                    0,
                    gl::RGB as i32,
                    self.img.width() as i32,
                    self.img.height() as i32,
                    0,
                    self.img.format(),
                    self.img.ty(),
                    self.img.ptr(),
                ));
            }
            _ => unimplemented!(),
        }

        if self.generate_mipmap {
            gl!(GenerateMipmap(Into::into(T)));
        }

        texture
    }

    pub fn generate_mipmap(mut self, gen: bool) -> Self {
        self.generate_mipmap = gen;
        self
    }

    pub fn wrap(self, coord: TextureCoordinate, wrap: TextureWrapping) -> Self {
        let key = match coord {
            TextureCoordinate::S => gl::TEXTURE_WRAP_S,
            TextureCoordinate::T => gl::TEXTURE_WRAP_T,
            TextureCoordinate::R => gl::TEXTURE_WRAP_R,
        };
        self.parameter(key, GLenum::from(wrap).into())
    }

    pub fn border_color(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.parameter(
            gl::TEXTURE_BORDER_COLOR,
            Parameter::Floats(vec![r, g, b, a]),
        )
    }

    pub fn minify_filtering(self, filtering: TextureFiltering) -> Self {
        self.parameter(gl::TEXTURE_MIN_FILTER, GLenum::from(filtering).into())
    }

    pub fn magnify_filtering(self, filtering: TextureFiltering) -> Self {
        self.parameter(gl::TEXTURE_MAG_FILTER, GLenum::from(filtering).into())
    }

    fn parameter(mut self, key: GLenum, value: Parameter) -> Self {
        self.parameters.insert(key, value);
        self
    }
}

pub type TextureBuilder2D<'a, I> = TextureBuilder<'a, I, { TextureType::Texture2D }>;
pub type TextureBuilder3D<'a, I> = TextureBuilder<'a, I, { TextureType::Texture3D }>;

pub trait Image {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn ptr(&self) -> *const c_void;
    fn format(&self) -> GLenum;
    fn ty(&self) -> GLenum;
}

#[cfg(feature = "image")]
impl Image for image::DynamicImage {
    fn width(&self) -> u32 {
        self.width()
    }

    fn height(&self) -> u32 {
        self.height()
    }

    fn ptr(&self) -> *const c_void {
        match self {
            image::DynamicImage::ImageRgb8(i) => (i as &[u8]).as_ptr() as *const c_void,
            image::DynamicImage::ImageRgba8(i) => (i as &[u8]).as_ptr() as *const c_void,
            image::DynamicImage::ImageRgb16(i) => (i as &[u16]).as_ptr() as *const c_void,
            image::DynamicImage::ImageRgba16(i) => (i as &[u16]).as_ptr() as *const c_void,
            x => unimplemented!("{:?}", x),
        }
    }

    fn format(&self) -> GLenum {
        match self {
            image::DynamicImage::ImageRgb8(_) | image::DynamicImage::ImageRgb16(_) => gl::RGB,
            image::DynamicImage::ImageRgba8(_) | image::DynamicImage::ImageRgba16(_) => gl::RGBA,
            x => unimplemented!("{:?}", x),
        }
    }

    fn ty(&self) -> GLenum {
        match self {
            image::DynamicImage::ImageRgb8(_) | image::DynamicImage::ImageRgba8(_) => {
                gl::UNSIGNED_BYTE
            }
            image::DynamicImage::ImageRgb16(_) | image::DynamicImage::ImageRgba16(_) => {
                gl::UNSIGNED_SHORT
            }
            x => unimplemented!("{:?}", x),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TextureCoordinate {
    S,
    T,
    R,
}

#[derive(Copy, Clone, Debug)]
pub enum TextureWrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

impl From<TextureWrapping> for GLenum {
    fn from(t: TextureWrapping) -> GLenum {
        match t {
            TextureWrapping::Repeat => gl::REPEAT,
            TextureWrapping::MirroredRepeat => gl::MIRRORED_REPEAT,
            TextureWrapping::ClampToEdge => gl::CLAMP_TO_EDGE,
            TextureWrapping::ClampToBorder => gl::CLAMP_TO_BORDER,
        }
    }
}

pub enum TextureFiltering {
    Nearest,
    Linear,
}

impl From<TextureFiltering> for GLenum {
    fn from(t: TextureFiltering) -> GLenum {
        match t {
            TextureFiltering::Nearest => gl::NEAREST,
            TextureFiltering::Linear => gl::LINEAR,
        }
    }
}

enum Parameter {
    //Float(GLfloat),
    Int(GLint),
    Floats(Vec<GLfloat>),
}

impl From<GLenum> for Parameter {
    fn from(e: GLenum) -> Parameter {
        Parameter::Int(e as GLint)
    }
}
