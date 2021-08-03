//////////////////////////////////////////////////
// Using

use game_gl::{ GameLoop, Runner, gl, Gl, InputEvent};


//////////////////////////////////////////////////
// Entry

use std::time::Instant;


pub fn start() {
    let mut game_loop = GameLoop::new(ExampleRunner{
        startTime: Instant::now()
    });
    game_loop.run();
}

pub struct ExampleRunner {
    startTime: Instant,
}

static BASE_V_LEFT: f32 = 1.25 * std::f32::consts::PI;
static BASE_V_RIGHT: f32 = 1.75 * std::f32::consts::PI;
static BASE_V_TOP: f32 = 0.5 * std::f32::consts::PI;
static SIDE_LEN: f32 = 0.5;

impl Runner for ExampleRunner {

    fn init(&mut self) {
        println!("init");
    }

    fn cleanup(&mut self) {
        println!("cleanup");
    }

    fn pause(&mut self) {
        println!("pause");
    }

    fn resume(&mut self) {
        println!("resume");
    }

    fn input(&mut self, input_events: &[InputEvent]) {
        input_events.iter().for_each(|input_event| {
            match input_event {
                _ => println!("input: {:?}", input_event)
            }
        });
    }

    fn update(&mut self, _elapsed_time: f32) {
        //println!("update (time: {}", elapsed_time);
    }

    fn render(&mut self, gl: &Gl) {
        ndk_glue::android_log_debug("rust_demo", "start render...");
        let time_elapsed = self.startTime.elapsed().as_millis();
        let percent = (time_elapsed % 5000) as f32 / 5000f32;
        let angle = percent * 2f32 * std::f32::consts::PI;
        println!("elapsed={}, percent = {}, angle={}, angle.sin={}",time_elapsed,percent,angle,angle.sin());

        unsafe {
            let vs = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl.CompileShader(vs);
    
            let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl.CompileShader(fs);
    
            let program = gl.CreateProgram();
            gl.AttachShader(program, vs);
            gl.AttachShader(program, fs);
            gl.LinkProgram(program);
            gl.UseProgram(program);

            // 可以释放
            gl.DeleteShader(vs);
            gl.DeleteShader(fs);
    
            let mut vb = std::mem::zeroed();
            gl.GenBuffers(1, &mut vb);
            gl.BindBuffer(gl::ARRAY_BUFFER, vb);
            let vertex = [
                SIDE_LEN * (BASE_V_LEFT+angle).cos(), SIDE_LEN * (BASE_V_LEFT+angle).sin(),  0.0,  0.4,  0.0, // position.x, position.y, v_color[r], v_color[g], v_color[b]
                SIDE_LEN * (BASE_V_TOP+angle).cos(),  SIDE_LEN * (BASE_V_TOP+angle).sin(),  0.0,  0.4,  0.0,
                SIDE_LEN * (BASE_V_RIGHT+angle).cos(), SIDE_LEN * (BASE_V_RIGHT+angle).sin(),  0.0,  0.4,  0.0,
            ];
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vertex.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertex.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
    
            if gl.BindVertexArray.is_loaded() {
                let mut vao = std::mem::zeroed();
                gl.GenVertexArrays(1, &mut vao);
                gl.BindVertexArray(vao);
            }
    
            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        }

        unsafe { 
            // gl.ClearColor(1.0, 0.0, 0.0, 1.0); 
            // gl.ClearDepthf(1.0);
            // gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.ClearColor(1.3 * (percent-0.5).abs(), 0., 1.3 * (0.5 - percent).abs(), 1.0); 
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
        println!("render finished!");
        ndk_glue::android_log_debug("rust_demo", "finish render...");
    }

    fn create_device(&mut self, _gl: &Gl) {
        println!("create_device");
    }

    fn destroy_device(&mut self, _gl: &Gl) {
        println!("destroy_device");
    }

    fn resize_device(&mut self, _gl: &Gl, _width: u32, _height: u32) {
        println!("resize_device");
    }
}


// #[rustfmt::skip]
// static VERTEX_DATA: [f32; 15] = [
//     -0.5, -0.5,  1.0,  0.0,  0.0, // position.x, position.y, v_color[r], v_color[g], v_color[b]
//      0.0,  0.5,  0.0,  1.0,  0.0,
//      0.5, -0.5,  0.0,  0.0,  1.0,
// ];

const VS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";