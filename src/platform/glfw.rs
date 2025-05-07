use glfw::Context;

use super::Window;

pub struct Framework {
    glfw: glfw::Glfw,
    width: usize,
    height: usize,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    texture: u32,
    shader_program: u32,
    vao: u32,
}

impl Window for Framework {
    fn new(buffer: &super::Buffer, title: &str) -> Self {
        // Initialize GLFW
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        // Configure GLFW
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(
                buffer.width as u32,
                buffer.height as u32,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);

        // Load OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Create shader program
        let shader_program = unsafe { create_shader_program() };

        // Set up vertex data and buffers
        let (vao, _vbo, _ebo) = unsafe { setup_vertices() };

        // Create texture
        let texture = unsafe { create_texture() };
        return Framework {
            glfw,
            window,
            events,
            width: buffer.width,
            height: buffer.height,
            shader_program,
            texture,
            vao,
        };
    }
    fn update(&mut self, buffer: &[u32]) {
        // Poll events
        self.glfw.poll_events();
        process_events(&mut self.window, &self.events);

        // Render
        unsafe {
            // Update texture with buffer data
            update_texture(
                self.texture,
                self.width as u32,
                self.height as u32,
                &buffer,
            );

            // Clear the screen
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw the texture
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        // Swap front and back buffers
        self.window.swap_buffers();
    }
    fn clean_up(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture);
            gl::DeleteProgram(self.shader_program);
        }
    }
    fn is_open(&self) -> bool {
        !self.window.should_close()
    }
}

// Vertex shader source
const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;
    
    out vec2 TexCoord;
    
    void main() {
        gl_Position = vec4(aPos, 1.0);
        TexCoord = aTexCoord;
    }
"#;

// Fragment shader source
const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    
    in vec2 TexCoord;
    
    uniform sampler2D ourTexture;
    
    void main() {
        FragColor = texture(ourTexture, TexCoord);
    }
"#;

fn process_events(
    window: &mut glfw::Window,
    events: &std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}

unsafe fn create_shader_program() -> u32 {
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert =
        std::ffi::CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
    gl::CompileShader(vertex_shader);
    check_shader_errors(vertex_shader, "VERTEX");

    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag =
        std::ffi::CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    gl::ShaderSource(
        fragment_shader,
        1,
        &c_str_frag.as_ptr(),
        std::ptr::null(),
    );
    gl::CompileShader(fragment_shader);
    check_shader_errors(fragment_shader, "FRAGMENT");

    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
    gl::LinkProgram(shader_program);
    check_program_errors(shader_program);

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    shader_program
}

unsafe fn check_shader_errors(shader: u32, shader_type: &str) {
    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut log_length = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
        let mut log = Vec::with_capacity(log_length as usize);
        log.set_len(log_length as usize - 1); // Subtract 1 to skip the null terminator
        gl::GetShaderInfoLog(
            shader,
            log_length,
            std::ptr::null_mut(),
            log.as_mut_ptr() as *mut i8,
        );
        let log_str = String::from_utf8_lossy(&log);
        eprintln!(
            "ERROR::SHADER::{}_COMPILATION_FAILED\n{}",
            shader_type, log_str
        );
    }
}

unsafe fn check_program_errors(program: u32) {
    let mut success = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success == 0 {
        let mut log_length = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
        let mut log = Vec::with_capacity(log_length as usize);
        log.set_len(log_length as usize - 1); // Subtract 1 to skip the null terminator
        gl::GetProgramInfoLog(
            program,
            log_length,
            std::ptr::null_mut(),
            log.as_mut_ptr() as *mut i8,
        );
        let log_str = String::from_utf8_lossy(&log);
        eprintln!("ERROR::PROGRAM::LINKING_FAILED\n{}", log_str);
    }
}

unsafe fn setup_vertices() -> (u32, u32, u32) {
    // Vertices for a quad (rectangle) that covers the screen
    #[rustfmt::skip]
    let vertices: [f32; 20] = [
        // positions       // texture coords (Y flipped)
         1.0,  1.0, 0.0,   1.0, 0.0, // top right
         1.0, -1.0, 0.0,   1.0, 1.0, // bottom right
        -1.0, -1.0, 0.0,   0.0, 1.0, // bottom left
        -1.0,  1.0, 0.0,   0.0, 0.0  // top left
    ];

    let indices: [u32; 6] = [
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
    ];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const std::ffi::c_void,
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * std::mem::size_of::<u32>()) as isize,
        indices.as_ptr() as *const std::ffi::c_void,
        gl::STATIC_DRAW,
    );

    // Position attribute
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        (5 * std::mem::size_of::<f32>()) as i32,
        std::ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    // Texture coord attribute
    gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        (5 * std::mem::size_of::<f32>()) as i32,
        (3 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
    );
    gl::EnableVertexAttribArray(1);

    (vao, vbo, ebo)
}

#[inline]
unsafe fn create_texture() -> u32 {
    let mut texture = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);

    // Set texture parameters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::NEAREST as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAG_FILTER,
        gl::NEAREST as i32,
    );

    texture
}
#[inline]
unsafe fn update_texture(
    texture: u32,
    width: u32,
    height: u32,
    buffer: &[u32],
) {
    gl::BindTexture(gl::TEXTURE_2D, texture);
    let rgba_buffer = argb_to_rgba(buffer);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        rgba_buffer.as_ptr() as *const std::ffi::c_void,
    );
}
#[inline(always)]
fn argb_to_rgba(input: &[u32]) -> Vec<u32> {
    input
        .iter()
        .map(|&argb| {
            let a = (argb >> 24) & 0xFF;
            let r = (argb >> 16) & 0xFF;
            let g = (argb >> 8) & 0xFF;
            let b = argb & 0xFF;
            (r as u32)
                | ((g as u32) << 8)
                | ((b as u32) << 16)
                | ((a as u32) << 24)
            // RGBA layout: 0xRRGGBBAA
        })
        .collect()
}
