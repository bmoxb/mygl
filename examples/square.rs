mod shared;
use shared::*;

use mygl::rendering::{DrawMode, IndexType};
use mygl::shaders::{FragmentShader, Shader, ShaderProgram, VertexShader};
use mygl::vao::{
    AttribPointerType, BufferUsageHint, ElementBufferObject, VertexArrayObjectBuilder,
    VertexBufferObject,
};

example!(square);

fn square(
    el: EventLoop<()>,
    windowed_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
) -> Result<(), mygl::Error> {
    mygl::debug::set_error_callback(error_callback);

    let vert = VertexShader::from_file("examples/shaders/triangle.vert")?;
    let frag = FragmentShader::from_file("examples/shaders/triangle.frag")?;
    let prog = ShaderProgram::new(vert, frag)?;

    let uniform = [0.0, 0.9, 0.2];
    prog.set_uniform("myColour", &uniform)?;

    // top right (0), bottom right (1), bottom left (2), top left (3)
    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];
    let vbo = VertexBufferObject::new(&vertices, BufferUsageHint::Static);

    let indices: [u32; 6] = [0, 1, 2, 0, 3, 2];
    let ebo = ElementBufferObject::new(&indices, BufferUsageHint::Static);

    let vao = VertexArrayObjectBuilder::new()
        .element_buffer_object(&ebo)
        .attrib_pointer(&vbo, 0, 3, AttribPointerType::Float, false, 3 * 4)
        .build();

    el.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::MainEventsCleared => {
            mygl::clear(0.8, 0.2, 0.2, 1.0);

            mygl::rendering::draw_elements(
                &prog,
                &vao,
                IndexType::UnsignedInt,
                DrawMode::Triangles,
                6,
            );

            windowed_context.swap_buffers().unwrap();
        }

        _ => (),
    });
}

fn error_callback(msg: &str) {
    eprintln!("OpenGL error: {}", msg);
}
