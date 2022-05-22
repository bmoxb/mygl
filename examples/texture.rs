mod shared;
use shared::*;

use mygl::rendering::DrawMode;
use mygl::shaders::{FragmentShader, Shader, ShaderProgram, VertexShader};
use mygl::textures::TextureBuilder2D;
use mygl::vao::{AttribPointerType, BufferUsageHint, VertexArrayObjectBuilder, VertexBufferObject};

example!(texture);

fn texture(
    el: EventLoop<()>,
    windowed_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
) -> Result<(), mygl::Error> {
    mygl::debug::set_error_callback(error_callback);

    let vert = VertexShader::from_file("examples/shaders/texture.vert")?;
    let frag = FragmentShader::from_file("examples/shaders/texture.frag")?;
    let prog = ShaderProgram::new(vert, frag)?;

    let data: [f32; 15] = [
        -0.5, -0.5, 0.0, 0.0, 0.0, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.5, 1.0,
    ];
    let vbo = VertexBufferObject::new(&data, BufferUsageHint::Static);

    let img = image::open("examples/images/bricks rgb8.png").unwrap();
    let texture = TextureBuilder2D::new(&img).generate_mipmap(true).build();
    prog.set_uniform("myTexture", /*&texture*/ 0)?;

    let vao = VertexArrayObjectBuilder::new()
        .attrib_pointer(&vbo, 0, 3, AttribPointerType::Float, false, 5 * 4, 0)
        .attrib_pointer(&vbo, 1, 2, AttribPointerType::Float, false, 5 * 4, 3 * 4)
        .build();

    el.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::MainEventsCleared => {
            mygl::clear(0.8, 0.2, 0.2, 1.0);

            mygl::rendering::draw_arrays_with_textures(
                &prog,
                &vao,
                DrawMode::Triangles,
                0,
                6,
                &[&texture],
            );

            windowed_context.swap_buffers().unwrap();
        }

        _ => (),
    });
}

fn error_callback(msg: &str) {
    eprintln!("OpenGL error: {}", msg);
}
