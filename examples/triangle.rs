mod shared;
use shared::*;

use mygl::rendering::DrawMode;
use mygl::shaders::{FragmentShader, ShaderProgram, VertexShader};
use mygl::vao::{
    BufferUsageHint, VertexArrayObjectBuilder, VertexAttribute, VertexAttributeType,
    VertexBufferObject,
};

example!(triangle);

fn triangle(
    el: EventLoop<()>,
    windowed_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
) -> Result<(), mygl::Error> {
    mygl::debug::set_error_callback(error_callback);

    let vert = VertexShader::from_file("examples/shaders/triangle.vert")?;
    let frag = FragmentShader::from_file("examples/shaders/triangle.frag")?;
    let prog = ShaderProgram::new(vert, frag)?;

    let uniform = [0.0, 0.2, 0.9];
    prog.set_uniform("myColour", &uniform)?;

    let mut triangle_height = -0.25;
    let data: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, triangle_height, 0.0];
    let vbo = VertexBufferObject::new(&data, BufferUsageHint::Dynamic);

    let vao = VertexArrayObjectBuilder::new()
        .attribute(
            &vbo,
            VertexAttribute {
                layout_index: 0,
                component_count: 3,
                component_type: VertexAttributeType::Float,
                normalize: false,
                stride: 3 * 4,
                offset: 0,
            },
        )
        .build();

    el.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::MainEventsCleared => {
            mygl::clear(0.8, 0.2, 0.2, 1.0);

            if triangle_height < 0.8 {
                triangle_height += 0.002;
                vbo.update_data(&[triangle_height], 7 * 4).unwrap();
            }

            mygl::rendering::draw_arrays(&prog, &vao, DrawMode::Triangles, 0, 6);

            windowed_context.swap_buffers().unwrap();
        }

        _ => (),
    });
}

fn error_callback(msg: &str) {
    eprintln!("OpenGL error: {}", msg);
}
