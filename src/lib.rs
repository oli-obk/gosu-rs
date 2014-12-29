#![feature(phase)]
extern crate gfx;
#[phase(plugin)]
extern crate gfx_macros;
extern crate glfw;

use gfx::{DeviceHelper, ToSlice};
use glfw::Context;

#[vertex_format]
#[deriving(Copy)]
struct Vertex {
  #[name = "a_Pos"]
  pos: [f32, ..2],

  #[name = "a_Color"]
  color: [f32, ..3],
}

static VERTEX_SRC: gfx::ShaderSource<'static> = shaders! {
  GLSL_120: b"
  #version 120

  attribute vec2 a_Pos;
  attribute vec3 a_Color;
  varying vec4 v_Color;

  void main() {
    v_Color = vec4(a_Color, 1.0);
    gl_Position = vec4(a_Pos, 0.0, 1.0);
  }
  "
  GLSL_150: b"
  #version 150 core

  in vec2 a_Pos;
  in vec3 a_Color;
  out vec4 v_Color;

  void main() {
    v_Color = vec4(a_Color, 1.0);
    gl_Position = vec4(a_Pos, 0.0, 1.0);
  }
  "
};

static FRAGMENT_SRC: gfx::ShaderSource<'static> = shaders! {
  GLSL_120: b"
  #version 120

  varying vec4 v_Color;

  void main() {
    gl_FragColor = v_Color;
  }
  "
  GLSL_150: b"
  #version 150 core

  in vec4 v_Color;
  out vec4 o_Color;

  void main() {
    o_Color = v_Color;
  }
  "
};


pub struct Window {
  graphics : gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>,
  window : glfw::Window,
}

impl Window {
  pub fn new(width : u32,
    height : u32) ->
    Window {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));

    let (window, events) = glfw
    .create_window(width, height, "no title", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

    window.make_current();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    window.set_key_polling(true);

    let (w, h) = window.get_framebuffer_size();
    let frame = gfx::Frame::new(w as u16, h as u16);

    let mut device = gfx::GlDevice::new(|s| window.get_proc_address(s));

    let vertex_data = [
      Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
      Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
      Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
    ];
    let mesh = device.create_mesh(&vertex_data);
    let slice = mesh.to_slice(gfx::PrimitiveType::TriangleList);

    let program = device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone())
      .unwrap();

    let mut graphics = gfx::Graphics::new(device);
    let batch: gfx::batch::RefBatch<(), ()> = graphics.make_batch(
      &program, &mesh, slice, &gfx::DrawState::new()).unwrap();

    let clear_data = gfx::ClearData {
      color: [0.3, 0.3, 0.3, 1.0],
      depth: 1.0,
      stencil: 0,
    };

    while !window.should_close() {
      glfw.poll_events();
      for (_, event) in glfw::flush_messages(&events) {
        match event {
          glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) =>
          window.set_should_close(true),
          _ => {},
        }
      }

      graphics.clear(clear_data, gfx::COLOR, &frame);
      graphics.draw(&batch, &(), &frame);
      graphics.end_frame();

      window.swap_buffers();
    }
    Window {
      window : window,
      graphics : graphics,
    }
  }
}
