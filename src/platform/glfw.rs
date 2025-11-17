use std::time::Duration;

use glfw::Action;
use glfw::Context;

use super::framework_traits::{
    ExtendedControl, ExtendedInput, ExtendedWindow, Output, Timing,
};
use super::{
    framework_traits::{Control, Input, Window},
    time::NativeTime,
    MouseButton, Time,
};
#[cfg(feature = "svg")]
use crate::platform::mouse::LoadCursorError;
#[cfg(target_os = "windows")]
use crate::platform::WindowLevel;
use crate::Buffer;
use crate::{
    extensions::*,
    graphics,
    platform::{
        framework_traits::Errors, keycodes::KeyCode,
        mouse::cursor_glfw::cursor_from_buffer,
    },
};
use crate::{
    platform::framework_traits::CursorStyleControl,
    system::action::{Decoration, Default},
};
/// glfw implementation of Framework
#[derive(Debug)]
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
#[allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]
fn log_errors(_: glfw::Error, description: String, _: &()) {
    println!("GLFW Error: {description}");
}

static LOG_ERRORS: Option<glfw::ErrorCallback<()>> = Some(glfw::Callback {
    f: log_errors,
    data: (),
});

impl Window for Framework {
    fn new(
        title: &str,
        settings: super::WindowSettings,
    ) -> Result<Self, Errors> {
        // Initialize GLFW
        let Ok(mut glfw) = glfw::init(LOG_ERRORS) else {
            return Err(Errors::Unknown);
        };
        // Configure GLFW
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // Create a windowed mode window and its OpenGL context
        let Some((mut window, events)) = glfw.create_window(
            settings.size.0 as u32,
            settings.size.1 as u32,
            title,
            glfw::WindowMode::Windowed,
        ) else {
            return Err(Errors::FailedToOpenWindow);
        };

        // Make the window's context current
        window.make_current();
        window.set_all_polling(false);
        window.set_key_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);

        window.set_pos(settings.position.0, settings.position.1);
        crate::system::Os::set_window_borderless(
            &get_native_window_handle_from_glfw(&window),
            settings.borderless,
        );
        crate::system::Os::set_window_position(
            &get_native_window_handle_from_glfw(&window),
            settings.position.0,
            settings.position.1,
        );
        crate::system::Os::set_window_level(
            &get_native_window_handle_from_glfw(&window),
            settings.window_level,
        );

        // Load OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol).cast());

        // Create shader program
        let shader_program = unsafe { create_shader_program() };

        // Set up vertex data and buffers
        let (vao, _vbo, _ebo) = unsafe { setup_vertices() };

        // Create texture
        let texture = unsafe { create_texture() };
        Ok(Self {
            glfw,
            window,
            events,
            width: settings.size.0 as usize,
            height: settings.size.1 as usize,
            shader_program,
            texture,
            vao,
            time: NativeTime::new(),
            keyboard_manager: super::shared::KeyManager::new(),
            mouse_manager: super::shared::MouseManager::new(),
            maximized: false,
            minimized: false,
        })
    }
    fn update(&mut self, buffer: &[u32]) -> Errors {
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
                buffer,
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
        Errors::AllGood
    }
    fn clean_up(&self) {
        unsafe {
            gl::DeleteTextures(1, &raw const self.texture);
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
    fn get_delta_time(&mut self) -> f64 {
        let (time, r) = super::shared::sample_fps(&self.time);
        self.time = time;
        r
    }
    #[inline]
    fn sleep(&self, time: Duration) {
        super::shared::sleep(time);
    }
}
use crate::system::action::Iconized;
use crate::system::Os;
impl ExtendedControl for Framework {
    #[inline]
    fn set_render_layer(&mut self, level: WindowLevel) {
        crate::system::Os::set_window_level(&self.get_window_handle(), level);
    }
    #[inline]
    fn maximize(&mut self) {
        Os::maximize(&self.get_window_handle());
    }
    #[inline]
    fn minimize(&mut self) {
        Os::minimize(&self.get_window_handle());
    }
    #[inline]
    fn restore(&mut self) {
        Os::restore(&self.get_window_handle());
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
    fn get_position(&self) -> (i32, i32) {
        self.window.get_pos()
    }
    fn get_size(&self) -> (i32, i32) {
        crate::system::Os::get_window_size(&get_native_window_handle_from_glfw(
            &self.window,
        ))
        .try_tuple_into()
        .unwrap_or((0, 0))
    }
    #[allow(clippy::cast_possible_wrap)]
    fn set_size(&mut self, buffer: &super::Buffer) {
        self.window.set_size(buffer.width as i32, buffer.height as i32);
    }
    fn set_position(&mut self, xy: (i32, i32)) {
        self.window.set_pos(xy.0, xy.1);
    }
}

impl Input for Framework {
    /// No, you won't get the real position of the mouse, calculate it yourself
    fn get_mouse_position(&self) -> Option<(i32, i32)> {
        self.window.get_cursor_pos().try_tuple_into()
        // let (mouse_x, mouse_y): (isize, isize) =
        //     self.window.get_cursor_pos().try_tuple_into();
        // let (window_x, window_y) = self.window.get_pos();
        // let relative_x = mouse_x - window_x as isize;
        // let relative_y = mouse_y - window_y as isize;
        // return Some((relative_x, relative_y));
    }
    fn is_key_down(&self, keycode: KeyCode) -> bool {
        self.keyboard_manager.is_key_pressed(keycode)
    }
    fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_manager.is_mouse_button_pressed(button)
    }
}
impl ExtendedInput for Framework {
    fn get_mouse_scroll(&self) -> Option<(f32, f32)> {
        Some(self.mouse_manager.get_scroll())
    }
    fn get_all_keys_down(&self) -> Vec<KeyCode> {
        self.keyboard_manager.get_all_pressed_keys()
    }
}
const fn action_to_bool(action: Action) -> Option<bool> {
    match action {
        Action::Press => Some(true),
        Action::Release => Some(false),
        Action::Repeat => None,
    }
}

impl Output for Framework {
    fn log(&self, t: &str) {
        super::shared::log(t);
    }
}

// const fn map_mouse(mouse_button: MouseButton) -> glfw::MouseButton {
//     match mouse_button {
//         MouseButton::Left => glfw::MouseButtonLeft,
//         MouseButton::Right => glfw::MouseButtonRight,
//         MouseButton::Middle => glfw::MouseButtonMiddle,
//         MouseButton::Unsupported => glfw::MouseButton::Button8,
//     }
// }

impl ExtendedWindow for Framework {
    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    /// Not yet implemented
    fn set_icon(&mut self, _buffer: &Buffer) -> Errors {
        Errors::NotImplemented
    }
    fn get_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        get_native_window_handle_from_glfw(&self.window)
    }
}
impl CursorStyleControl for Framework {
    #[cfg(feature = "svg")]
    fn set_cursor_style(&mut self, style: &super::Cursor) -> Errors {
        //println!("Setting cursor style");
        super::mouse::use_cursor(style, Some(&mut self.window))
    }
    #[cfg(feature = "svg")]
    fn load_custom_cursors(
        &mut self,
        size: crate::extensions::U2,
        main_color: u32,
        secondary_color: u32,
    ) -> Result<super::mouse::Cursors, LoadCursorError> {
        super::mouse::Cursors::load(
            size,
            main_color,
            secondary_color,
            super::mouse::cursor_glfw::load_base_cursor_with_file,
        )
        .map_err(|x| {
            LoadCursorError::InvalidImageData(format!("Unable to access {x}"))
        })
    }
    fn load_custom_cursor(
        &mut self,
        image: super::Buffer,
        hotspot: (u8, u8),
    ) -> Result<super::mouse::Cursor, LoadCursorError> {
        Ok(cursor_from_buffer(image, unsafe {
            hotspot.try_tuple_into().unwrap_unchecked()
        }))
    }
}

// Vertex shader source
const VERTEX_SHADER_SOURCE: &str = r"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;
    
    out vec2 TexCoord;
    
    void main() {
        gl_Position = vec4(aPos, 1.0);
        TexCoord = aTexCoord;
    }
";

// Fragment shader source
const FRAGMENT_SHADER_SOURCE: &str = r"
    #version 330 core
    out vec4 FragColor;
    
    in vec2 TexCoord;
    
    uniform sampler2D ourTexture;
    
    void main() {
        FragColor = texture(ourTexture, TexCoord);
    }
";

fn process_events(window: &mut Framework) {
    let events: &std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)> =
        &window.events;
    window.mouse_manager.reset_scroll();

    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(key, _scan_code, action, _mods) => {
                if let Some(converted) = action_to_bool(action) {
                    window
                        .keyboard_manager
                        .set_key_state(map_glfw_key_to_keycode(key), converted);
                }
            }
            glfw::WindowEvent::Scroll(x, y) => {
                window
                    .mouse_manager
                    .add_scroll((x, y).try_tuple_into().unwrap_or_default());
            }
            glfw::WindowEvent::Close => {
                window.window.set_should_close(true);
            }
            glfw::WindowEvent::MouseButton(button, action, _mods) => {
                if let Some(action) = action_to_bool(action) {
                    window.mouse_manager.set_mouse_button_state(
                        map_glfw_mouse_button_to_mouse_button(button),
                        action,
                    );
                }
            }
            glfw::WindowEvent::Maximize(bool) => {
                window.minimized = !bool;
            }
            glfw::WindowEvent::Iconify(bool) => {
                window.minimized = bool;
            }

            _ => {}
        }
    }
}
#[must_use]
/// Convert a glfw mouse button to a mirl mouse buttons
pub const fn map_glfw_mouse_button_to_mouse_button(
    button: glfw::MouseButton,
) -> MouseButton {
    match button {
        glfw::MouseButton::Button1 => MouseButton::Left,
        glfw::MouseButton::Button2 => MouseButton::Right,
        glfw::MouseButton::Button3 => MouseButton::Middle,
        glfw::MouseButton::Button4
        | glfw::MouseButton::Button5
        | glfw::MouseButton::Button6
        | glfw::MouseButton::Button7
        | glfw::MouseButton::Button8 => MouseButton::Unsupported,
    }
}
#[must_use]
/// Convert a glfw keycode to a mirl keycode
#[allow(clippy::too_many_lines)]
pub const fn map_glfw_key_to_keycode(key: glfw::Key) -> KeyCode {
    match key {
        glfw::Key::A => KeyCode::A,
        glfw::Key::B => KeyCode::B,
        glfw::Key::C => KeyCode::C,
        glfw::Key::D => KeyCode::D,
        glfw::Key::E => KeyCode::E,
        glfw::Key::F => KeyCode::F,
        glfw::Key::G => KeyCode::G,
        glfw::Key::H => KeyCode::H,
        glfw::Key::I => KeyCode::I,
        glfw::Key::J => KeyCode::J,
        glfw::Key::K => KeyCode::K,
        glfw::Key::L => KeyCode::L,
        glfw::Key::M => KeyCode::M,
        glfw::Key::N => KeyCode::N,
        glfw::Key::O => KeyCode::O,
        glfw::Key::P => KeyCode::P,
        glfw::Key::Q => KeyCode::Q,
        glfw::Key::R => KeyCode::R,
        glfw::Key::S => KeyCode::S,
        glfw::Key::T => KeyCode::T,
        glfw::Key::U => KeyCode::U,
        glfw::Key::V => KeyCode::V,
        glfw::Key::W => KeyCode::W,
        glfw::Key::X => KeyCode::X,
        glfw::Key::Y => KeyCode::Y,
        glfw::Key::Z => KeyCode::Z,
        glfw::Key::Escape => KeyCode::Escape,
        glfw::Key::F1 => KeyCode::F1,
        glfw::Key::F2 => KeyCode::F2,
        glfw::Key::F3 => KeyCode::F3,
        glfw::Key::F4 => KeyCode::F4,
        glfw::Key::F5 => KeyCode::F5,
        glfw::Key::F6 => KeyCode::F6,
        glfw::Key::F7 => KeyCode::F7,
        glfw::Key::F8 => KeyCode::F8,
        glfw::Key::F9 => KeyCode::F9,
        glfw::Key::F10 => KeyCode::F10,
        glfw::Key::F11 => KeyCode::F11,
        glfw::Key::F12 => KeyCode::F12,
        glfw::Key::Space => KeyCode::Space,
        glfw::Key::Enter => KeyCode::Enter,
        glfw::Key::Backspace => KeyCode::Backspace,
        glfw::Key::Left => KeyCode::LeftArrow,
        glfw::Key::Right => KeyCode::RightArrow,
        glfw::Key::Up => KeyCode::UpArrow,
        glfw::Key::Down => KeyCode::DownArrow,
        glfw::Key::Tab => KeyCode::Tab,
        glfw::Key::LeftShift => KeyCode::LeftShift,
        glfw::Key::RightShift => KeyCode::RightShift,
        glfw::Key::LeftControl => KeyCode::LeftControl,
        glfw::Key::RightControl => KeyCode::RightControl,
        glfw::Key::LeftAlt => KeyCode::LeftAlt,
        glfw::Key::RightAlt => KeyCode::RightAlt,
        glfw::Key::LeftSuper => KeyCode::LeftSuper,
        glfw::Key::RightSuper => KeyCode::RightSuper,
        glfw::Key::Comma => KeyCode::Comma,
        glfw::Key::Minus => KeyCode::Minus,
        glfw::Key::Period => KeyCode::Period,
        glfw::Key::Slash => KeyCode::Slash,
        glfw::Key::Semicolon => KeyCode::Semicolon,
        glfw::Key::LeftBracket => KeyCode::LeftBracket,
        glfw::Key::Backslash => KeyCode::Backslash,
        glfw::Key::RightBracket => KeyCode::RightBracket,
        glfw::Key::F13 => KeyCode::F13,
        glfw::Key::F14 => KeyCode::F14,
        glfw::Key::F15 => KeyCode::F15,
        glfw::Key::F16 => KeyCode::F16,
        glfw::Key::F17 => KeyCode::F17,
        glfw::Key::F18 => KeyCode::F18,
        glfw::Key::F19 => KeyCode::F19,
        glfw::Key::F20 => KeyCode::F20,
        glfw::Key::F21 => KeyCode::F21,
        glfw::Key::F22 => KeyCode::F22,
        glfw::Key::F23 => KeyCode::F23,
        glfw::Key::F24 => KeyCode::F24,
        glfw::Key::F25 => KeyCode::F25,
        glfw::Key::Apostrophe => KeyCode::Apostrophe,
        glfw::Key::CapsLock => KeyCode::CapsLock,
        glfw::Key::Delete => KeyCode::Delete,
        glfw::Key::End => KeyCode::End,
        glfw::Key::Home => KeyCode::Home,
        glfw::Key::Insert => KeyCode::Insert,
        glfw::Key::Kp0 => KeyCode::KeyPad0,
        glfw::Key::Kp1 => KeyCode::KeyPad1,
        glfw::Key::Kp2 => KeyCode::KeyPad2,
        glfw::Key::Kp3 => KeyCode::KeyPad3,
        glfw::Key::Kp4 => KeyCode::KeyPad4,
        glfw::Key::Kp5 => KeyCode::KeyPad5,
        glfw::Key::Kp6 => KeyCode::KeyPad6,
        glfw::Key::Kp7 => KeyCode::KeyPad7,
        glfw::Key::Kp8 => KeyCode::KeyPad8,
        glfw::Key::Kp9 => KeyCode::KeyPad9,
        glfw::Key::KpDivide => KeyCode::KeyPadDivide,
        glfw::Key::KpEnter => KeyCode::KeyPadEnter,
        glfw::Key::KpEqual => KeyCode::KeyPadEqual,
        glfw::Key::KpMultiply => KeyCode::KeyPadMultiply,
        glfw::Key::KpSubtract => KeyCode::KeyPadSubtract,
        glfw::Key::GraveAccent => KeyCode::Grave,
        glfw::Key::KpAdd => KeyCode::KeyPadAdd,
        glfw::Key::KpDecimal => KeyCode::KeyPadDecimal,
        glfw::Key::Num0 => KeyCode::Num0,
        glfw::Key::Num1 => KeyCode::Num1,
        glfw::Key::Num2 => KeyCode::Num2,
        glfw::Key::Num3 => KeyCode::Num3,
        glfw::Key::Num4 => KeyCode::Num4,
        glfw::Key::Num5 => KeyCode::Num5,
        glfw::Key::Num6 => KeyCode::Num6,
        glfw::Key::Num7 => KeyCode::Num7,
        glfw::Key::Num8 => KeyCode::Num8,
        glfw::Key::Num9 => KeyCode::Num9,
        glfw::Key::Menu => KeyCode::Menu,
        glfw::Key::NumLock => KeyCode::NumLock,
        glfw::Key::PageDown => KeyCode::PageDown,
        glfw::Key::PageUp => KeyCode::PageUp,
        glfw::Key::Pause => KeyCode::Pause,
        glfw::Key::PrintScreen => KeyCode::PrintScreen,
        glfw::Key::World1 => KeyCode::World1,
        glfw::Key::World2 => KeyCode::World2,
        glfw::Key::Equal => KeyCode::Equal,
        glfw::Key::ScrollLock => KeyCode::ScrollLock,
        glfw::Key::Unknown => KeyCode::Unknown,
    }
}

unsafe fn create_shader_program() -> u32 {
    let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    let c_str_vert =
        std::ffi::CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap_unchecked();
    gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
    gl::CompileShader(vertex_shader);
    check_shader_errors(vertex_shader, "VERTEX");

    let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    let c_str_frag =
        std::ffi::CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap_unchecked();
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

// unsafe fn check_shader_errors(shader: u32, shader_type: &str) {
//     let mut success = 0;
//     gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
//     if success == 0 {
//         let mut log_length = 0;
//         gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
//         let mut log = Vec::with_capacity(log_length as usize);
//         log.set_len(log_length as usize - 1); // Subtract 1 to skip the null terminator
//         gl::GetShaderInfoLog(
//             shader,
//             log_length,
//             std::ptr::null_mut(),
//             log.as_mut_ptr() as *mut i8,
//         );
//         let log_str = String::from_utf8_lossy(&log);
//         eprintln!(
//             "ERROR::SHADER::{}_COMPILATION_FAILED\n{}",
//             shader_type, log_str
//         );
//     }
// }
unsafe fn check_shader_errors(shader: u32, shader_type: &str) {
    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &raw mut success);
    if success == 0 {
        let mut log_length = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &raw mut log_length);

        let mut log = vec![0u8; log_length as usize]; // properly initialized

        gl::GetShaderInfoLog(
            shader,
            log_length,
            std::ptr::null_mut(),
            log.as_mut_ptr().cast::<i8>(),
        );

        let log_str = String::from_utf8_lossy(&log);
        eprintln!("ERROR::SHADER::{shader_type}_COMPILATION_FAILED\n{log_str}");
    }
}

// unsafe fn check_program_errors(program: u32) {
//     let mut success = 0;
//     gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
//     if success == 0 {
//         let mut log_length = 0;
//         gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
//         let mut log = Vec::with_capacity(log_length as usize);
//         log.set_len(log_length as usize - 1); // Subtract 1 to skip the null terminator
//         gl::GetProgramInfoLog(
//             program,
//             log_length,
//             std::ptr::null_mut(),
//             log.as_mut_ptr() as *mut i8,
//         );
//         let log_str = String::from_utf8_lossy(&log);
//         eprintln!("ERROR::PROGRAM::LINKING_FAILED\n{}", log_str);
//     }
// }
unsafe fn check_program_errors(program: u32) {
    let mut success = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &raw mut success);
    if success == 0 {
        let mut log_length = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &raw mut log_length);

        let mut log = vec![0u8; log_length as usize]; // initialized to 0

        gl::GetProgramInfoLog(
            program,
            log_length,
            std::ptr::null_mut(),
            log.as_mut_ptr().cast::<i8>(),
        );

        // Strip trailing null terminator if present
        if log.last() == Some(&0) {
            log.pop();
        }

        let log_str = String::from_utf8_lossy(&log);
        eprintln!("ERROR::PROGRAM::LINKING_FAILED\n{log_str}");
    }
}
#[allow(clippy::cast_possible_wrap)]
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

    gl::GenVertexArrays(1, &raw mut vao);
    gl::GenBuffers(1, &raw mut vbo);
    gl::GenBuffers(1, &raw mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr().cast::<std::ffi::c_void>(),
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * std::mem::size_of::<u32>()) as isize,
        indices.as_ptr().cast::<std::ffi::c_void>(),
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

#[allow(clippy::cast_possible_wrap)]
#[inline]
unsafe fn create_texture() -> u32 {
    let mut texture = 0;
    gl::GenTextures(1, &raw mut texture);
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
#[allow(clippy::cast_possible_wrap)]
unsafe fn update_texture(
    texture: u32,
    width: u32,
    height: u32,
    buffer: &[u32],
) {
    gl::BindTexture(gl::TEXTURE_2D, texture);
    let rgba_buffer = graphics::switch_red_and_blue_list(buffer);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        rgba_buffer.as_ptr().cast::<std::ffi::c_void>(),
    );
}
#[allow(
    clippy::cast_possible_wrap,
    clippy::unwrap_used,
    clippy::missing_panics_doc
)]
/// Get the window handle from glfw
#[must_use]
pub fn get_native_window_handle_from_glfw(
    window: &glfw::Window,
) -> raw_window_handle::RawWindowHandle {
    #[cfg(target_os = "windows")]
    {
        let handle = raw_window_handle::Win32WindowHandle::new(
            std::num::NonZero::new(window.get_win32_window() as isize).unwrap(),
        );
        raw_window_handle::RawWindowHandle::Win32(handle)
    }

    #[cfg(target_os = "macos")]
    {
        let mut handle = raw_window_handle::AppKitWindowHandle::empty();
        handle.ns_view = window.get_cocoa_window();
        raw_window_handle::RawWindowHandle::AppKit(handle)
    }

    // #[cfg(all(target_os = "linux", not(feature = "wayland")))]
    // {
    //     let mut handle = raw_window_handle::XlibWindowHandle::empty();
    //     handle.window = window.get_x11_window();
    //     handle.display = window.get_x11_display().cast();
    //     raw_window_handle::RawWindowHandle::Xlib(handle)
    // }

    // #[cfg(all(target_os = "linux", feature = "wayland"))]
    // {
    //     let mut handle = raw_window_handle::WaylandWindowHandle::empty();
    //     handle.surface = window.get_wayland_window();
    //     handle.display = window.get_wayland_display();
    //     raw_window_handle::RawWindowHandle::Wayland(handle)
    // }
}
