extern crate gl;
extern crate sdl2;

pub mod shader;

use std::ffi::CString;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    gl_attr.set_red_size(8u8);
    gl_attr.set_green_size(8u8);
    gl_attr.set_blue_size(8u8);
    gl_attr.set_depth_size(16u8);

    gl_attr.set_multisample_buffers(1u8);
    gl_attr.set_multisample_samples(8u8);

    let(wnd_width, wnd_height) = (800, 800);
    let window = video_subsystem
        .window("Game", wnd_width, wnd_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vert_shader =
        shader::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        shader::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = shader::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();

    unsafe {
        gl::Viewport(0, 0, wnd_width as i32, wnd_height as i32);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // set up vertex buffer object
    let raudis = 0.8f32;
    let vertices: Vec<f32> = vec![
        // positions                // colors
        ((000f32 / 180f32) * std::f32::consts::PI).cos() * raudis,
        ((000f32 / 180f32) * std::f32::consts::PI).sin() * raudis,
        1.0,
        0.0,
        0.0,
        ((120f32 / 180f32) * std::f32::consts::PI).cos() * raudis,
        ((120f32 / 180f32) * std::f32::consts::PI).sin() * raudis,
        0.0,
        1.0,
        0.0,
        ((240f32 / 180f32) * std::f32::consts::PI).cos() * raudis,
        ((240f32 / 180f32) * std::f32::consts::PI).sin() * raudis,
        0.0,
        0.0,
        1.0,
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    let proj_mat_location: gl::types::GLint;
    let model_mat_location: gl::types::GLint;

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let pos_attrib = gl::GetAttribLocation(
            shader_program.id,
            CString::new("Position")
                .unwrap()
                .as_bytes_with_nul()
                .as_ptr() as *const i8,
        ) as gl::types::GLuint;
        gl::EnableVertexAttribArray(pos_attrib); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            pos_attrib, // index of the generic vertex attribute ("layout (location = 0)")
            2,          // the number of components per generic vertex attribute
            gl::FLOAT,  // data type
            gl::FALSE,  // normalized (int-to-float conversion)
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        let color_attrib = gl::GetAttribLocation(
            shader_program.id,
            CString::new("Color").unwrap().as_bytes_with_nul().as_ptr() as *const i8,
        ) as gl::types::GLuint;
        gl::EnableVertexAttribArray(color_attrib); // this is "layout (location = 1)" in vertex shader
        gl::VertexAttribPointer(
            color_attrib, // index of the generic vertex attribute ("layout (location = 1)")
            3,            // the number of components per generic vertex attribute
            gl::FLOAT,    // data type
            gl::FALSE,    // normalized (int-to-float conversion)
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        proj_mat_location = gl::GetUniformLocation(
            shader_program.id,
            CString::new("proj_mat_location")
                .unwrap()
                .as_bytes_with_nul()
                .as_ptr() as *const i8,
        );
        model_mat_location = gl::GetUniformLocation(
            shader_program.id,
            CString::new("model_mat_location")
                .unwrap()
                .as_bytes_with_nul()
                .as_ptr() as *const i8,
        );
    }

    resize(wnd_width as i32, wnd_height as i32, proj_mat_location);

    let mut timer = sdl.timer().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => resize(w, h, proj_mat_location),
                _ => {}
            }
        }

        // render window contents here
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::BindVertexArray(vao);
            set_rotation_matrix(
                (timer.ticks() as f32) / 20000.0f32 * std::f32::consts::PI / 2.0f32,
                model_mat_location,
            );
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}

fn set_rotation_matrix(rad: f32, model_mat_location: gl::types::GLint) {
    // rotation around z axis
    let sin_angle = rad.sin();
    let cos_angle = rad.cos();
    let mut mat = [0f32; 16];
    mat[0] = cos_angle;
    mat[1] = sin_angle;
    mat[2] = 0.0;
    mat[3] = 0.0;

    mat[4] = -sin_angle;
    mat[5] = cos_angle;
    mat[6] = 0.0;
    mat[7] = 0.0;

    mat[8] = 0.0;
    mat[9] = 0.0;
    mat[10] = 1.0;
    mat[11] = 0.0;

    mat[12] = 0.0;
    mat[13] = 0.0;
    mat[14] = 0.0;
    mat[15] = 1.0;
    unsafe {
        gl::UniformMatrix4fv(
            model_mat_location,
            1,
            0 as gl::types::GLboolean,
            mat.as_ptr() as *const gl::types::GLfloat,
        );
    }
}

fn resize(w: i32, h: i32, proj_mat_location: gl::types::GLint) {
    unsafe {
        gl::Viewport(0, 0, w, h);
    }
    // set orthogonal view so that coordinates [-1, 1] area always visible and proportional on x and y axis
    if w > h {
        let f = w as f32 / h as f32;
        set_ortho_matrix(-f, f, -1.0, 1.0, -1.0, 1.0, proj_mat_location);
    } else {
        let f = h as f32 / w as f32;
        set_ortho_matrix(-1.0, 1.0, -f, f, -1.0, 1.0, proj_mat_location);
    }
}
fn set_ortho_matrix(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    n: f32,
    f: f32,
    proj_mat_location: gl::types::GLint,
) {
    // set orthogonal matrix
    let mut mat = [0f32; 16];
    mat[0] = 2.0 / (right - left);
    mat[1] = 0.0;
    mat[2] = 0.0;
    mat[3] = 0.0;

    mat[4] = 0.0;
    mat[5] = 2.0 / (top - bottom);
    mat[6] = 0.0;
    mat[7] = 0.0;

    mat[8] = 0.0;
    mat[9] = 0.0;
    mat[10] = -2.0 / (f - n);
    mat[11] = 0.0;

    mat[12] = -(right + left) / (right - left);
    mat[13] = -(top + bottom) / (top - bottom);
    mat[14] = -(f + n) / (f - n);
    mat[15] = 1.0;
    unsafe {
        gl::UniformMatrix4fv(
            proj_mat_location,
            1,
            0 as gl::types::GLboolean,
            mat.as_ptr() as *const gl::types::GLfloat,
        );
    }
}
