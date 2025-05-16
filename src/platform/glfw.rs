use glfw::{Action, Context};

use crate::graphics::argb_list_to_rgba_list;

use super::{
    framework_traits::{
        Control, ExtendedControl, ExtendedInput, ExtendedWindow, Input, Output,
        Timing, Window,
    },
    time::NativeTime,
    MouseButton, Time,
};

pub struct Framework {
    glfw: glfw::Glfw,
    width: usize,
    height: usize,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    texture: u32,
    shader_program: u32,
    vao: u32,
    time: NativeTime,
    keyboard_manager: super::shared::KeyManager,
    mouse_manager: super::shared::MouseManager,
    maximized: bool,
    minimized: bool,
}
pub fn log_errors(_: glfw::Error, description: String, _: &()) {
    println!("GLFW Error: {}", description);
}

pub static LOG_ERRORS: Option<glfw::ErrorCallback<()>> = Some(glfw::Callback {
    f: log_errors as fn(glfw::Error, String, &()),
    data: (),
});

impl Window for Framework {
    fn new(buffer: &super::Buffer, title: &str) -> Self {
        // Initialize GLFW
        let mut glfw = glfw::init(LOG_ERRORS).unwrap();
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
            time: NativeTime::new(),
            keyboard_manager: super::shared::KeyManager::new(),
            mouse_manager: super::shared::MouseManager::new(),
            maximized: false,
            minimized: false,
        };
    }
    fn update(&mut self, buffer: &[u32]) {
        // Poll events
        self.glfw.poll_events();

        process_events(self);

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

impl Timing for Framework {
    #[inline]
    fn get_time(&self) -> Box<dyn Time> {
        super::shared::get_time()
    }
    #[inline]
    fn sample_fps(&mut self) -> u64 {
        let (time, r) = super::shared::sample_fps(&self.time);
        self.time = time;
        return r;
    }
    #[inline]
    fn sleep(&self, time: u64) {
        super::shared::sleep(time);
    }
}

#[cfg(target_os = "windows")]
impl ExtendedControl for Framework {
    #[inline]
    fn set_always_ontop(&mut self, always_ontop: bool) {
        super::shared::always_ontop(
            &self.window.get_win32_window(),
            always_ontop,
        );
    }
    #[inline]
    fn maximize(&mut self) {
        super::shared::maximize(&self.window.get_win32_window());
    }
    #[inline]
    fn minimize(&mut self) {
        super::shared::minimize(&self.window.get_win32_window());
    }
    #[inline]
    fn restore(&mut self) {
        super::shared::restore(&self.window.get_win32_window());
    }
    #[inline]
    fn is_maximized(&self) -> bool {
        self.maximized
    }
    #[inline]
    fn is_minimized(&self) -> bool {
        self.minimized
    }
}

impl Control for Framework {
    fn get_position(&self) -> (isize, isize) {
        let (x, y) = self.window.get_pos();
        return (x as isize, y as isize);
    }
    fn get_size(&self) -> (usize, usize) {
        let (x, y) = self.window.get_size();
        return (x as usize, y as usize);
    }
    fn set_size(&mut self, buffer: &super::Buffer) {
        self.window.set_size(buffer.width as i32, buffer.height as i32);
    }
    fn set_position(&mut self, x: isize, y: isize) {
        self.window.set_pos(x as i32, y as i32)
    }
}

impl Input for Framework {
    /// No, you won't get the real position of the mouse, calculate it yourself
    fn get_mouse_position(&self) -> Option<(f64, f64)> {
        let (mouse_x, mouse_y) = self.window.get_cursor_pos();
        let (window_x, window_y) = self.window.get_pos();
        let relative_x = mouse_x - window_x as f64;
        let relative_y = mouse_y - window_y as f64;
        return Some((relative_x, relative_y));
    }
    fn is_key_down(&self, keycode: super::KeyCode) -> bool {
        self.keyboard_manager.is_key_pressed(keycode)
    }
    fn is_mouse_down(&self, button: super::MouseButton) -> bool {
        self.mouse_manager.is_mouse_button_pressed(button)
    }
}
impl ExtendedInput for Framework {
    fn get_mouse_scroll(&self) -> Option<(f64, f64)> {
        Some(self.mouse_manager.get_scroll())
    }
}
const fn action_to_bool(action: Action) -> bool {
    match action {
        Action::Press => true,
        Action::Release => false,
        Action::Repeat => false, //???
    }
}

impl Output for Framework {
    fn log<T: std::fmt::Debug>(&self, t: T) {
        super::shared::log(t)
    }
}

const fn map_mouse(mouse_button: MouseButton) -> glfw::MouseButton {
    match mouse_button {
        MouseButton::Left => glfw::MouseButtonLeft,
        MouseButton::Right => glfw::MouseButtonRight,
        MouseButton::Middle => glfw::MouseButtonMiddle,
        MouseButton::Unsupported => glfw::MouseButton::Button8,
    }
}

// use glfw::{CursorImage, Glfw};

// let width = 16;
// let height = 16;
// let pixels = vec![255u8; width * height * 4]; // Solid white 16x16 RGBA

// let image = CursorImage {
//     width: width as u32,
//     height: height as u32,
//     pixels,
// };

// let cursor = glfw.create_cursor(&image, 0, 0);
// window.set_cursor(Some(&cursor));

impl ExtendedWindow for Framework {
    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
    fn set_cursor_style(&mut self, style: &super::Cursor) {
        println!("Setting cursor style");
        super::cursors::use_cursor(style, Some(&mut self.window));
    }
    /// Not yet implemented
    /// #[deprecated(note = "This function is not implemented and should not be used.")]
    fn set_icon(&mut self, _buffer: &[u32], _width: u32, _height: u32) {
        panic!("Not yet implemented");
    }
    fn load_custom_cursor(
        &mut self,
        size: crate::render::U2,
        main_color: u32,
        secondary_color: u32,
    ) -> super::cursors::Cursors {
        super::cursors::Cursors::load(
            size,
            main_color,
            secondary_color,
            super::cursors::cursor_glfw::load_base_cursor_with_file,
        )
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

fn process_events(window: &mut Framework) {
    let events: &std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)> =
        &window.events;
    window.mouse_manager.reset_scroll();

    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(key, _scancode, action, _mods) => {
                window.keyboard_manager.set_key_state(
                    map_glfw_key_to_keycode(key),
                    action_to_bool(action),
                );
            }
            glfw::WindowEvent::Scroll(x, y) => {
                window.mouse_manager.add_scroll(x, y);
            }
            glfw::WindowEvent::Close => {
                window.window.set_should_close(true);
            }
            glfw::WindowEvent::MouseButton(button, action, _mods) => {
                window.mouse_manager.set_mouse_button_state(
                    map_glfw_mouse_button_to_mouse_button(button),
                    action_to_bool(action),
                );
            }
            glfw::WindowEvent::Maximize(bool) => {
                window.minimized = bool;
            }
            glfw::WindowEvent::Iconify(bool) => {
                window.minimized = bool;
            }

            _ => {}
        }
    }
}

const fn map_glfw_mouse_button_to_mouse_button(
    button: glfw::MouseButton,
) -> super::MouseButton {
    match button {
        glfw::MouseButton::Button1 => super::MouseButton::Left,
        glfw::MouseButton::Button2 => super::MouseButton::Right,
        glfw::MouseButton::Button3 => super::MouseButton::Middle,
        glfw::MouseButton::Button4 => super::MouseButton::Unsupported,
        glfw::MouseButton::Button5 => super::MouseButton::Unsupported,
        glfw::MouseButton::Button6 => super::MouseButton::Unsupported,
        glfw::MouseButton::Button7 => super::MouseButton::Unsupported,
        glfw::MouseButton::Button8 => super::MouseButton::Unsupported,
    }
}

const fn map_glfw_key_to_keycode(key: glfw::Key) -> super::KeyCode {
    match key {
        glfw::Key::A => super::KeyCode::A,
        glfw::Key::B => super::KeyCode::B,
        glfw::Key::C => super::KeyCode::C,
        glfw::Key::D => super::KeyCode::D,
        glfw::Key::E => super::KeyCode::E,
        glfw::Key::F => super::KeyCode::F,
        glfw::Key::G => super::KeyCode::G,
        glfw::Key::H => super::KeyCode::H,
        glfw::Key::I => super::KeyCode::I,
        glfw::Key::J => super::KeyCode::J,
        glfw::Key::K => super::KeyCode::K,
        glfw::Key::L => super::KeyCode::L,
        glfw::Key::M => super::KeyCode::M,
        glfw::Key::N => super::KeyCode::N,
        glfw::Key::O => super::KeyCode::O,
        glfw::Key::P => super::KeyCode::P,
        glfw::Key::Q => super::KeyCode::Q,
        glfw::Key::R => super::KeyCode::R,
        glfw::Key::S => super::KeyCode::S,
        glfw::Key::T => super::KeyCode::T,
        glfw::Key::U => super::KeyCode::U,
        glfw::Key::V => super::KeyCode::V,
        glfw::Key::W => super::KeyCode::W,
        glfw::Key::X => super::KeyCode::X,
        glfw::Key::Y => super::KeyCode::Y,
        glfw::Key::Z => super::KeyCode::Z,
        glfw::Key::Escape => super::KeyCode::Escape,
        glfw::Key::F1 => super::KeyCode::F1,
        glfw::Key::F2 => super::KeyCode::F2,
        glfw::Key::F3 => super::KeyCode::F3,
        glfw::Key::F4 => super::KeyCode::F4,
        glfw::Key::F5 => super::KeyCode::F5,
        glfw::Key::F6 => super::KeyCode::F6,
        glfw::Key::F7 => super::KeyCode::F7,
        glfw::Key::F8 => super::KeyCode::F8,
        glfw::Key::F9 => super::KeyCode::F9,
        glfw::Key::F10 => super::KeyCode::F10,
        glfw::Key::F11 => super::KeyCode::F11,
        glfw::Key::F12 => super::KeyCode::F12,
        glfw::Key::Space => super::KeyCode::Space,
        glfw::Key::Enter => super::KeyCode::Enter,
        glfw::Key::Backspace => super::KeyCode::Backspace,
        glfw::Key::Left => super::KeyCode::Left,
        glfw::Key::Right => super::KeyCode::Right,
        glfw::Key::Up => super::KeyCode::Up,
        glfw::Key::Down => super::KeyCode::Down,
        glfw::Key::Tab => super::KeyCode::Tab,
        glfw::Key::LeftShift => super::KeyCode::LeftShift,
        glfw::Key::RightShift => super::KeyCode::RightShift,
        glfw::Key::LeftControl => super::KeyCode::LeftControl,
        glfw::Key::RightControl => super::KeyCode::RightControl,
        glfw::Key::LeftAlt => super::KeyCode::LeftAlt,
        glfw::Key::RightAlt => super::KeyCode::RightAlt,
        glfw::Key::LeftSuper => super::KeyCode::LeftSuper,
        glfw::Key::RightSuper => super::KeyCode::RightSuper,
        glfw::Key::Comma => super::KeyCode::Comma,
        glfw::Key::Minus => super::KeyCode::Minus,
        glfw::Key::Period => super::KeyCode::Period,
        glfw::Key::Slash => super::KeyCode::Slash,
        glfw::Key::Semicolon => super::KeyCode::Semicolon,
        glfw::Key::LeftBracket => super::KeyCode::LeftBracket,
        glfw::Key::Backslash => super::KeyCode::Backslash,
        glfw::Key::RightBracket => super::KeyCode::RightBracket,
        glfw::Key::F13 => super::KeyCode::F13,
        glfw::Key::F14 => super::KeyCode::F14,
        glfw::Key::F15 => super::KeyCode::F15,
        glfw::Key::F16 => super::KeyCode::F16,
        glfw::Key::F17 => super::KeyCode::F17,
        glfw::Key::F18 => super::KeyCode::F18,
        glfw::Key::F19 => super::KeyCode::F19,
        glfw::Key::F20 => super::KeyCode::F20,
        glfw::Key::F21 => super::KeyCode::F21,
        glfw::Key::F22 => super::KeyCode::F22,
        glfw::Key::F23 => super::KeyCode::F23,
        glfw::Key::F24 => super::KeyCode::F24,
        glfw::Key::F25 => super::KeyCode::F25,
        glfw::Key::Apostrophe => super::KeyCode::Apostrophe,
        glfw::Key::CapsLock => super::KeyCode::CapsLock,
        glfw::Key::Delete => super::KeyCode::Delete,
        glfw::Key::End => super::KeyCode::End,
        glfw::Key::Home => super::KeyCode::Home,
        glfw::Key::Insert => super::KeyCode::Insert,
        glfw::Key::Kp0 => super::KeyCode::KeyPad0,
        glfw::Key::Kp1 => super::KeyCode::KeyPad1,
        glfw::Key::Kp2 => super::KeyCode::KeyPad2,
        glfw::Key::Kp3 => super::KeyCode::KeyPad3,
        glfw::Key::Kp4 => super::KeyCode::KeyPad4,
        glfw::Key::Kp5 => super::KeyCode::KeyPad5,
        glfw::Key::Kp6 => super::KeyCode::KeyPad6,
        glfw::Key::Kp7 => super::KeyCode::KeyPad7,
        glfw::Key::Kp8 => super::KeyCode::KeyPad8,
        glfw::Key::Kp9 => super::KeyCode::KeyPad9,
        glfw::Key::KpDivide => super::KeyCode::KeyPadDivide,
        glfw::Key::KpEnter => super::KeyCode::KeyPadEnter,
        glfw::Key::KpEqual => super::KeyCode::KeyPadEqual,
        glfw::Key::KpMultiply => super::KeyCode::KeyPadMultiply,
        glfw::Key::KpSubtract => super::KeyCode::KeyPadSubtract,
        glfw::Key::GraveAccent => super::KeyCode::Grave,
        glfw::Key::KpAdd => super::KeyCode::KeyPadAdd,
        glfw::Key::KpDecimal => super::KeyCode::KeyPadDecimal,
        glfw::Key::Num0 => super::KeyCode::Num0,
        glfw::Key::Num1 => super::KeyCode::Num1,
        glfw::Key::Num2 => super::KeyCode::Num2,
        glfw::Key::Num3 => super::KeyCode::Num3,
        glfw::Key::Num4 => super::KeyCode::Num4,
        glfw::Key::Num5 => super::KeyCode::Num5,
        glfw::Key::Num6 => super::KeyCode::Num6,
        glfw::Key::Num7 => super::KeyCode::Num7,
        glfw::Key::Num8 => super::KeyCode::Num8,
        glfw::Key::Num9 => super::KeyCode::Num9,
        glfw::Key::Menu => super::KeyCode::Menu,
        glfw::Key::NumLock => super::KeyCode::NumLock,
        glfw::Key::PageDown => super::KeyCode::PageDown,
        glfw::Key::PageUp => super::KeyCode::PageUp,
        glfw::Key::Pause => super::KeyCode::Pause,
        glfw::Key::PrintScreen => super::KeyCode::PrintScreen,
        glfw::Key::World1 => super::KeyCode::World1,
        glfw::Key::World2 => super::KeyCode::World2,
        glfw::Key::Equal => super::KeyCode::Equal,
        glfw::Key::ScrollLock => super::KeyCode::ScrollLock,
        glfw::Key::Unknown => super::KeyCode::Unknown,
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
    let rgba_buffer = argb_list_to_rgba_list(buffer);
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
