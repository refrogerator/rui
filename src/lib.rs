use glow::*;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Mod;
use swash::scale::Source;

pub mod widgets;
pub mod prelude;
pub mod messages;
pub mod keyvalues;

use widgets::{ColumnContainer, Rect, Widget};
use widgets::Label;
use widgets::Button;
use widgets::Offset;
use widgets::Anchor;
use widgets::Layout;
use widgets::SingleContainer;
use widgets::RowContainer;
use widgets::Color;
use widgets::IVec2;

struct Glyph {
    tex: Texture,
    bearing: (i32, i32),
    x: i32,
    y: i32,
    advance: f32,
}

#[derive(Debug)]
struct UiColors {
    foreground: Color,
    background: Color,
    unfocused_text: Color,
    current_line: Color,
    cursor: Color,
}

struct LoadedFont {
    size: i32,
    glyphs: Vec<Glyph>,
    max_advance: (i16, i16),
    ascent: i32,
}

impl LoadedFont {}

fn create_shader(gl: &glow::Context, vert: &str, frag: &str) -> glow::Program {
    unsafe {
        let program = gl.create_program().unwrap();
        let vs_source = vert;
        let fs_source = frag;
        let vs = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vs, &vs_source);
        gl.compile_shader(vs);
        if !gl.get_shader_compile_status(vs) {
            panic!("{}", gl.get_shader_info_log(vs));
        }
        gl.attach_shader(program, vs);

        let fs = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fs, &fs_source);
        gl.compile_shader(fs);
        if !gl.get_shader_compile_status(fs) {
            panic!("{}", gl.get_shader_info_log(fs));
        }
        gl.attach_shader(program, fs);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        gl.detach_shader(program, vs);
        gl.delete_shader(vs);

        gl.detach_shader(program, fs);
        gl.delete_shader(fs);
        return program;
    }
}

pub struct DrawingContext {
    pub gl: glow::Context,
    pub gl_context: sdl2::video::GLContext,
    pub video: sdl2::VideoSubsystem,
    pub quad_shader: glow::Program,
    pub outline_quad_shader: glow::Program,
    pub text_shader: glow::Program,
    pub loaded_fonts: Vec<LoadedFont>,
    pub current_font: usize,
    pub sdl: sdl2::Sdl,
    pub mouse_state: sdl2::mouse::MouseState,
    pub window: sdl2::video::Window,
}

impl DrawingContext {
    pub fn draw_quad(&self, dims: &widgets::Rect, color: &Color) {
        let window_size = self.window.drawable_size();
        unsafe {
            self.gl.use_program(Some(self.quad_shader));
            self.gl.uniform_2_f32(
                self.gl
                    .get_uniform_location(self.quad_shader, "offset")
                    .as_ref(),
                dims.x / window_size.0 as f32,
                dims.y / window_size.1 as f32,
            );
            self.gl.uniform_2_f32(
                self.gl
                    .get_uniform_location(self.quad_shader, "scale")
                    .as_ref(),
                dims.w / window_size.0 as f32,
                dims.h / window_size.1 as f32,
            );
            self.gl.uniform_4_f32(
                self.gl
                    .get_uniform_location(self.quad_shader, "color")
                    .as_ref(),
                color.r,
                color.g,
                color.b,
                1.0,
            );
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
    pub fn draw_rounded_quad(&self, dims: &widgets::Rect, color: &Color, rounding: f32) {
        let window_size = self.window.drawable_size();
        unsafe {
            self.gl.use_program(Some(self.quad_shader));
            self.gl.uniform_2_f32(
                self.gl
                    .get_uniform_location(self.quad_shader, "offset")
                    .as_ref(),
                dims.x / window_size.0 as f32,
                dims.y / window_size.1 as f32,
            );
            self.gl.uniform_2_f32(
                self.gl
                    .get_uniform_location(self.quad_shader, "scale")
                    .as_ref(),
                dims.w / window_size.0 as f32,
                dims.h / window_size.1 as f32,
            );
            self.gl.uniform_4_f32(
                self.gl
                    .get_uniform_location(self.quad_shader, "color")
                    .as_ref(),
                color.r,
                color.g,
                color.b,
                1.0,
            );
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
    pub fn draw_rounded_quad_outline(&self, dims: &widgets::Rect, color: &Color, rounding: f32, border_thickness: f32) {
        //let window_size = self.window.drawable_size();
        self.draw_rounded_quad(&Rect { x: dims.x + border_thickness, y: dims.y, w: dims.w - border_thickness, h: border_thickness }, color, rounding);
        self.draw_rounded_quad(&Rect { x: dims.x + dims.w - border_thickness, y: border_thickness + dims.y, w: border_thickness, h: dims.h - border_thickness }, color, rounding);
        self.draw_rounded_quad(&Rect { x: dims.x, y: dims.y + dims.h - border_thickness, w: dims.w - border_thickness, h: border_thickness }, color, rounding);
        self.draw_rounded_quad(&Rect { x: dims.x, y: dims.y, w: border_thickness, h: dims.h - border_thickness }, color, rounding);
        //unsafe {
        //    self.gl.use_program(Some(self.outline_quad_shader));
        //    self.gl.uniform_2_f32(
        //        self.gl
        //            .get_uniform_location(self.outline_quad_shader, "offset")
        //            .as_ref(),
        //        dims.x / window_size.0 as f32,
        //        dims.y / window_size.1 as f32,
        //    );
        //    self.gl.uniform_2_f32(
        //        self.gl
        //            .get_uniform_location(self.outline_quad_shader, "scale")
        //            .as_ref(),
        //        dims.w / window_size.0 as f32,
        //        dims.h / window_size.1 as f32,
        //    );
        //    self.gl.uniform_4_f32(
        //        self.gl
        //            .get_uniform_location(self.outline_quad_shader, "color")
        //            .as_ref(),
        //        color.r,
        //        color.g,
        //        color.b,
        //        1.0,
        //    );
        //    self.gl.uniform_1_f32(
        //        self.gl
        //            .get_uniform_location(self.outline_quad_shader, "border_width")
        //            .as_ref(),
        //        border_thickness,
        //    );
        //    self.gl.uniform_2_f32(
        //        self.gl
        //            .get_uniform_location(self.outline_quad_shader, "dims")
        //            .as_ref(),
        //        window_size.0 as f32,
        //        window_size.1 as f32,
        //    );
        //    self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        //}
    }
    pub fn draw_glyph(&self, ch: char, pos: IVec2, color: &Color) -> f32 {
        if (ch as usize) < 32 {
            dbg!(ch);
        }
        let glyph = &self.get_current_font().glyphs[ch as usize - 32];
        let window_size = self.window.drawable_size();
        let pen = (pos.x as f32, pos.y as f32);
        // println!("{:?}", pen);

        unsafe {
            self.gl.use_program(Some(self.text_shader));
            self.gl.enable(glow::BLEND);
            self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            self.gl.bind_texture(glow::TEXTURE_2D, Some(glyph.tex));
            // println!("{}", self.current_font.ascent);
            // println!("{:?}", ((pen.0 + glyph.bearing.0 as f32), (pen.1 + (glyph.y - glyph.bearing.1 + self.current_font.ascent) as f32)));
            self.gl.uniform_2_f32(
                self.gl
                    .get_uniform_location(self.text_shader, "offset")
                    .as_ref(),
                (pen.0 + glyph.bearing.0 as f32) / window_size.0 as f32,
                (pen.1 + (glyph.y + self.get_current_font().ascent - glyph.bearing.1) as f32)
                    / window_size.1 as f32,
            );
            self.gl.uniform_2_f32(
                self.gl
                    .get_uniform_location(self.text_shader, "scale")
                    .as_ref(),
                (glyph.x as f32) / window_size.0 as f32,
                (-glyph.y as f32) / window_size.1 as f32,
            );
            self.gl.uniform_4_f32(
                self.gl
                    .get_uniform_location(self.text_shader, "color")
                    .as_ref(),
                color.r,
                color.g,
                color.b,
                1.0,
            );
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);

            self.gl.disable(glow::BLEND);
        }
        
        glyph.advance
    }

    pub fn get_current_font(&self) -> &LoadedFont {
        &self.loaded_fonts[self.current_font]
    }

    pub fn draw_glyph_on_grid(&self, ch: char, pos: (i32, i32), color: &Color) {
        let block_size = self.get_current_font().max_advance;
        let pen = (
            (pos.0 * block_size.0 as i32) as i32,
            (pos.1 * block_size.1 as i32) as i32,
        );

        self.draw_glyph(ch, IVec2 { x: pen.0, y: pen.1 }, color);
    }

    pub fn draw_text(&self, text: &str, mut pos: IVec2, color: &Color) -> IVec2 {
        for i in text.chars() {
            pos.x += self.draw_glyph(i, pos.clone(), color) as i32;
        }
        pos
    }

    pub fn draw_text_monospace(&self, text: &str, mut pos: IVec2, color: &Color) -> IVec2 {
        for i in text.chars() {
            self.draw_glyph(i, pos.clone(), color);
            pos.x += self.get_current_font().max_advance.0 as i32;
        }
        pos
    }

    pub fn get_window_size(&self) -> IVec2 {
        let size = self.window.drawable_size();

        IVec2::new(size.0 as i32, size.1 as i32)
    }
    pub fn get_mouse_pos(&self) -> IVec2 {
        IVec2::new(self.mouse_state.x(), self.mouse_state.y())
    }
    pub fn get_mouse_button_pressed(&self, button: sdl2::mouse::MouseButton) -> bool {
        self.mouse_state.is_mouse_button_pressed(button)
    }
}

pub trait App {
    fn handle_command(&mut self, root: WidgetRootRef, cmd: String);
    fn update(&mut self, root: WidgetRootRef);
}

#[macro_export]
macro_rules! window {
    ( $handler:expr, $keyvalues:expr, $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Box<dyn Widget>> = Vec::new();
            $(temp_vec.push(Box::new($x));)*
            Window::new(temp_vec, $handler, $keyvalues)
        }
    };
}

#[derive(Clone, Debug)]
pub struct WidgetRootRef {
    pub root: std::sync::Arc<std::sync::RwLock<WidgetRoot>>
}

impl WidgetRootRef {
    pub fn get(&self, key: &str) -> Option<Value> {
        self.root.read().unwrap().keyvalues.get(key).cloned()
    }
    pub fn modify(&self, key: &str, f: impl Fn(&mut Value)) {
        let mut root = self.root.write().unwrap();
        f(root.keyvalues.get_mut(key).unwrap());
    }
    pub fn modify_keyvalue(&self, key: &str, f: impl Fn(&mut KeyValues)) {
        let mut root = self.root.write().unwrap();
        if let Value::KeyValue(ref mut i) = root.keyvalues.get_mut(key).unwrap() {
            f(i);
        }        
    }
    pub fn modify_bool(&self, key: &str, f: impl Fn(&mut bool)) {
        let mut root = self.root.write().unwrap();
        if let Value::Bool(ref mut i) = root.keyvalues.get_mut(key).unwrap() {
            f(i);
        }        
    }
    pub fn modify_int(&self, key: &str, f: impl Fn(&mut i32)) {
        let mut root = self.root.write().unwrap();
        if let Value::Int(ref mut i) = root.keyvalues.get_mut(key).unwrap() {
            f(i);
        }        
    }
    pub fn modify_float(&self, key: &str, f: impl Fn(&mut f32)) {
        let mut root = self.root.write().unwrap();
        if let Value::Float(ref mut i) = root.keyvalues.get_mut(key).unwrap() {
            f(i);
        }        
    }
    pub fn modify_string(&self, key: &str, f: impl Fn(&mut String)) {
        let mut root = self.root.write().unwrap();
        if let Value::String(ref mut i) = root.keyvalues.get_mut(key).unwrap() {
            f(i);
        }        
    }
    pub fn modify_array(&self, key: &str, f: impl Fn(&mut Vec<Value>)) {
        let mut root = self.root.write().unwrap();
        if let Value::Array(ref mut i) = root.keyvalues.get_mut(key).unwrap() {
            f(i);
        }        
    }
}

pub type KeyValues = std::collections::HashMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Value {
    KeyValue(KeyValues),
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Array(Vec<Value>),
}

impl Value {
    fn to_str(&self) -> String {
        match self {
            Value::Int(val) => format!("{}", val),
            Value::Float(val) => format!("{}", val),
            Value::String(val) => format!("{}", val),
            Value::Bool(val) => format!("{}", val),
            _ => { String::new() }
        }
    }
}

#[derive(Debug)]
pub struct WidgetRoot {
    pub keyvalues: KeyValues,
}

pub struct Window<T: App> {
    pub context: DrawingContext,
    pub widget_root: WidgetRootRef,
    pub widgets: Vec<Box<dyn Widget>>,
    pub handler: T,
}

impl<T: App> Window<T> {
    pub fn new(widgets: Vec<Box<dyn Widget>>, handler: T, keyvalues: KeyValues) -> Self {
        sdl2::hint::set_video_minimize_on_focus_loss(false);
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);
        let window = video
            .window("rui", 640, 480)
            .opengl()
            .resizable()
            .position_centered()
            .allow_highdpi()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        video.gl_set_swap_interval(sdl2::video::SwapInterval::VSync).unwrap();
        let gl = unsafe {
            glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
        };

        // gl_attr.set_multisample_samples(4);

        //let vertex_buffer = unsafe { gl.create_vertex_array().expect("uh nuh uh") };

        let test_shader = create_shader(&gl, include_str!("test.vert"), include_str!("test.frag"));

        let quad_shader = create_shader(&gl, include_str!("quad.vert"), include_str!("quad.frag"));
        let outline_shader = create_shader(&gl, include_str!("quad.vert"), include_str!("outline.frag"));
        let text_shader = create_shader(&gl, include_str!("quad.vert"), include_str!("text.frag"));

        let mut scontext = swash::scale::ScaleContext::new();
        let file = std::fs::read("/usr/share/fonts/liberation-sans/LiberationSans-Regular.ttf").unwrap();
        let text_size = 20;
        let font_size = text_size as f32;
        let font = swash::FontRef::from_index(&file, 0).unwrap();
        let mut scaler = scontext.builder(font).size(font_size).hint(false).build();

        let mut shape_context = swash::shape::ShapeContext::new();
        let mut shaper = shape_context
            .builder(font)
            .script(swash::text::Script::Latin)
            .direction(swash::shape::Direction::LeftToRight)
            .size(font_size)
            .build();

        let mut glyphs = Vec::new();
        // let glyph = font.charmap().map(33 as u32);
        // let bitmap =
        //     swash::scale::Render::new(&[
        //     Source::Bitmap(swash::scale::StrikeWith::BestFit),
        //     Source::Outline
        // ]).format(swash::zeno::Format::Alpha).render(&mut scaler, glyph).unwrap();
        let metrics = font.metrics(&[]).scale(font_size);
        let glyph_metrics = font.glyph_metrics(&[]).scale(font_size);
        //println!("{:?}", metrics);
        let max_advance = (
            metrics.max_width as i16,
            (metrics.ascent + metrics.descent) as i16,
        );
        for i in 32..128 {
            let glyph = font.charmap().map(i as u32);
            let bitmap = swash::scale::Render::new(&[
                Source::Bitmap(swash::scale::StrikeWith::BestFit),
                Source::Outline,
            ])
            .format(swash::zeno::Format::Alpha)
            .render(&mut scaler, glyph)
            .unwrap();
            // let bitmap = scaler.scale_color_bitmap(glyph, swash::scale::StrikeWith::BestFit);

            unsafe {
                gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
                let tex = gl.create_texture().unwrap();
                gl.bind_texture(glow::TEXTURE_2D, Some(tex));
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::R8 as i32,
                    bitmap.placement.width as i32,
                    bitmap.placement.height as i32,
                    0,
                    glow::RED,
                    glow::UNSIGNED_BYTE,
                    Some(bitmap.data.as_ref()),
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::LINEAR as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::LINEAR as i32,
                );
                glyphs.push(Glyph {
                    tex,
                    bearing: (bitmap.placement.left, bitmap.placement.top),
                    x: bitmap.placement.width as i32,
                    y: bitmap.placement.height as i32,
                    advance: glyph_metrics.advance_width(glyph)
                });
            }
        }

        let font = LoadedFont {
            size: font_size as i32,
            glyphs,
            max_advance,
            ascent: metrics.ascent as i32,
        };
        let loaded_fonts = vec![font];

        video.text_input().start();
        video.text_input().stop();
        let background_color = Color::from_hex("000000");

        unsafe {
            gl.use_program(Some(text_shader));
            let background_color_rgb = background_color.srgb_to_rgb();
            gl.clear_color(
                background_color_rgb.r,
                background_color_rgb.g,
                background_color_rgb.b,
                1.0,
            );

            gl.enable(glow::FRAMEBUFFER_SRGB);
        }

        let mouse_state = {
            sdl2::mouse::MouseState::new(&sdl.event_pump().unwrap())
        };

        let context = DrawingContext {
            gl,
            gl_context,
            video,
            quad_shader,
            outline_quad_shader: outline_shader,
            text_shader,
            loaded_fonts,
            current_font: 0,
            sdl,
            mouse_state, 
            window,
        };
        Window {
            context,
            widget_root: WidgetRootRef { root: std::sync::Arc::new(std::sync::RwLock::new(WidgetRoot {
                keyvalues,
            }))},
            widgets,
            handler
        }
    }
    pub fn post_message(&mut self, msg: String) {
        for widget in self.widgets.iter_mut() {
            //widget.handle_message(&msg);
        }
    }
    pub fn run(&mut self) {
        let mut quit = false;
        let mut old = std::time::Instant::now();
        let mut events = self.context.sdl.event_pump().unwrap();

        for widget in self.widgets.iter_mut() {
            widget.init(&widgets::WidgetBase { root: self.widget_root.clone(), local: KeyValues::new() });
        }

        while !quit {
            let window_size = self.context.window.drawable_size();
            let window_rect = widgets::Rect { x: 0.0, y: 0.0, w: window_size.0 as f32, h: window_size.1 as f32 };

            let delta = old.elapsed().as_secs_f32();
            old = std::time::Instant::now();
            for event in events.poll_iter() {
                for widget in self.widgets.iter_mut() {
                    let chud = widget.handle_input(&mut self.context, &event, &window_rect);
                    if !chud.is_empty() {
                        //println!("{:?}", chud);
                        for cmd in chud {
                            self.handler.handle_command(self.widget_root.clone(), cmd);
                        }
                    }
                }
                match event {
                    sdl2::event::Event::Quit { .. } => quit = true,
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(x, y) => unsafe {
                            self.context.gl.viewport(0, 0, x, y)
                        },
                        _ => {}
                    },
                    _ => {}
                }
            }
            self.context.mouse_state = sdl2::mouse::MouseState::new(&events);

            unsafe {
                self.context.gl.clear(glow::COLOR_BUFFER_BIT);

                for widget in self.widgets.iter_mut() {
                    let chud = widget.render(&mut self.context, &window_rect);
                }

                self.context.window.gl_swap_window();
            }
        }
    }
}

