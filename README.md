# MyGL

MyGL is a high-level Rust wrapper around OpenGL FFI bindings. It is the aim of this library to abstract away the state machine that OpenGL is designed around and instead represent OpenGL objects, operations, and functionality in a more modern, 'Rust-y' way.

MyGL is in on-going development - see the section below for a list of planned (as well as already implemented) features. It is not intended that the library will ever implement every OpenGL feature but enough features will be covered such that the library could be used as a back-end for a majority of small and medium-size graphical applications.

The **nightly** compiler toolchain is required due to the use of enums in constant generics ([rust-lang/rust #95174](https://github.com/rust-lang/rust/issues/95174)). Once this is stabilised, MyGL will likely return to using the stable toolchain.

## Roadmap

* [ ] Shader programs
  * [x] Load, compile, and link shaders into shader programs
  * [x] Set uniforms (scalars, vectors, matrices)
  * [ ] Geometry shaders
* [x] Buffer objects
  * [x] Load data
  * [x] Update data (`glBufferSubData`)
  * [x] Resize/overwrite/clear data
* [x] Textures
* [x] Rendering
  * [x] Draw arrays
  * [x] Draw elements
  * [x] Draw textures
* [ ] Debugging features
  * [x] Allow setting a callback to be called on OpenGL error
  * [ ] Log all OpenGL operations
* [ ] Comprehensive documentation
* [ ] Comprehensive tests
* [ ] Full `nalgebra` support
* [ ] Full `image` support
* Examples
  * [x] Basic 2D drawing (triangles and squares, with and without textures)
  * [ ] Basic 3D drawing (textured cubes)
  * [ ] Render a 3D teapot
