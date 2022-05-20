# MyGL

MyGL is a high-level Rust wrapper around OpenGL FFI bindings. It is the aim of this library to abstract away the state machine that OpenGL is designed around and instead represent OpenGL objects, operations, and functionality in a more modern, 'Rust-y' way.

MyGL is in on-going development - see the section below for a list of planned (as well as already implemented) features. It is not intended that the library will ever implement every OpenGL feature but enough features will be covered such that the library could be used as a back-end for a majority of small and medium-size graphical applications.

## Roadmap

* [ ] Shader programs
  * [x] Load, compile, and link shaders into shader programs
  * [x] Set uniforms (scalars, vectors, matrices)
  * [ ] Geometry shaders
* [ ] Buffer objects
  * [x] Load data
  * [ ] Update data (`glBufferSubData`)
  * [ ] Resize/overwrite/clear data
* [ ] Textures
* [ ] Rendering
  * [x] Draw arrays
  * [ ] Draw elements
  * [ ] Draw textures
* [ ] Debugging features
  * [x] Allow setting a callback to be called on OpenGL error
  * [ ] Log all OpenGL operations
* [ ] Full documentation
* [ ] End-to-end tests
