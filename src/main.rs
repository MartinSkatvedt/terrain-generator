extern crate nalgebra_glm as glm;
use std::{
    cmp::Ordering,
    ptr,
    sync::{Arc, Mutex},
    thread,
};

use chunk_container::ChunkContainer;
use curve_editor::curve::Curve;
use glutin::event::{
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;

pub mod light;
use light::point_light::PointLight;
use light::point_light_settings::PointLightSettings;
pub mod scenenode;
use imgui::Condition;
use material::{material_settings::MaterialSettings, Material};
use mesh::mesh_settings::MeshSettings;
use noise_map::noise_map_settings;
use scenenode::SceneNode;
pub mod chunk;
pub mod chunk_container;
pub mod curve_editor;
pub mod material;
pub mod mesh;
pub mod noise_map;
pub mod shader;
pub mod triangle;
pub mod utils;
pub mod vertex;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

const VIEW_DISTANCE: f32 = 400.0;

unsafe fn draw_scene(
    nodes: &Vec<scenenode::SceneNode>,
    view_projection_matrix: &glm::Mat4,
    light: &PointLight,
    cam_pos: &glm::Vec3,
) {
    for node in nodes {
        if node.vao_id == 0 {
            continue;
        }
        let mut model_matrix = glm::Mat4::identity();
        model_matrix = glm::translation(&glm::vec3(
            node.reference_point.x * -1.0,
            node.reference_point.y * -1.0,
            node.reference_point.z * -1.0,
        )) * model_matrix;

        model_matrix = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0)) * model_matrix;
        model_matrix = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0)) * model_matrix;
        model_matrix = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0)) * model_matrix;

        model_matrix = glm::scale(&model_matrix, &node.scale);

        model_matrix = glm::translation(&node.reference_point) * model_matrix;
        model_matrix = glm::translation(&node.position) * model_matrix;

        let transformation_matrix: glm::Mat4 = view_projection_matrix * model_matrix;

        gl::UseProgram(node.shader_program);
        gl::BindVertexArray(node.vao_id);

        gl::UniformMatrix4fv(10, 1, gl::TRUE, transformation_matrix.as_ptr());
        gl::UniformMatrix4fv(11, 1, gl::TRUE, model_matrix.as_ptr());
        gl::Uniform3fv(12, 1, cam_pos.as_ptr());

        gl::Uniform3fv(13, 1, light.position.as_ptr());
        gl::Uniform3fv(14, 1, light.ambient.as_ptr());
        gl::Uniform3fv(15, 1, light.diffuse.as_ptr());
        gl::Uniform3fv(16, 1, light.specular.as_ptr());

        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("terrain-generator")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));

    let context_builder = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = context_builder
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe {
        let c = windowed_context.make_current().unwrap();
        gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
        c
    };

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut winit_platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    winit_platform.attach_window(
        imgui.io_mut(),
        &context.window(),
        imgui_winit_support::HiDpiMode::Rounded,
    );

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| context.get_proc_address(s) as _);

    let mut pressed_keys = Vec::<VirtualKeyCode>::with_capacity(10);
    let mut window_size = (INITIAL_SCREEN_W, INITIAL_SCREEN_H, false);
    let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

    // Set up openGL
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::CULL_FACE);
        gl::Disable(gl::MULTISAMPLE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
    }

    let shape_shader = unsafe {
        shader::ShaderBuilder::new()
            .attach_file("./shaders/shape.vert")
            .attach_file("./shaders/shape.frag")
            .link()
    };

    let cubic_curve = Curve::cubic();
    let mut mesh_settings = MeshSettings::new(" Mesh".to_string(), 10.0, cubic_curve, 0);
    let mut noise_map_settings = noise_map_settings::NoiseMapSettings::new();

    let mut point_light_settings = PointLightSettings {
        name: "Point Light".to_string(),
        position: glm::vec3(0.0, 10.0, 0.0),
        ambient: glm::vec3(0.7, 0.7, 0.7),
        diffuse: glm::vec3(0.2, 0.2, 0.2),
        specular: glm::vec3(0.4, 0.4, 0.4),
    };

    let mut point_light_source = point_light_settings.get_point_light();

    let water_material_settings = MaterialSettings::standard_water_material();
    let sand_material_settings = MaterialSettings::standard_sand_material();
    let grass_material_settings = MaterialSettings::standard_grass_material();
    let snow_material_settings = MaterialSettings::standard_snow_material();

    let grass_material = Material::new(&grass_material_settings);
    let sand_material = Material::new(&sand_material_settings);
    let water_material = Material::new(&water_material_settings);
    let snow_material = Material::new(&snow_material_settings);

    let mut materials = vec![water_material, sand_material, grass_material, snow_material];
    let mut material_settings = vec![
        water_material_settings,
        sand_material_settings,
        grass_material_settings,
        snow_material_settings,
    ];

    let mut chunk_container = ChunkContainer::new(241, 200.0);

    let first_frame_time = std::time::Instant::now();
    let mut previous_frame_time = first_frame_time;

    let mut cam_pos: glm::Vec3 = glm::vec3(0.0, 4.0, 5.0);
    let mut cam_dir: glm::Vec3 = glm::vec3(1.0, 0.0, 1.0);
    let mut yaw: f32 = -90.0;
    let mut pitch: f32 = 0.0;

    let mut cam_front: glm::Vec3 = glm::vec3(0.0, 0.0, -1.0);
    let cam_up: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);

    let move_speed: f32 = 50.0;
    let cam_speed: f32 = 100.0;

    chunk_container.generate_visible_chunks(cam_pos, &materials);
    chunk_container.update_current_visible_chunks(&materials, &noise_map_settings, &mesh_settings);
    chunk_container.wait_for_all_threads_to_finish();

    // Start the event loop -- This is where window events are initially handled
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!(
                    "New window size! width: {}, height: {}",
                    physical_size.width, physical_size.height
                );
                window_size = (physical_size.width, physical_size.height, true);
            }

            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                match key_state {
                    Released => {
                        if pressed_keys.contains(&keycode) {
                            let i = pressed_keys.iter().position(|&k| k == keycode).unwrap();
                            pressed_keys.remove(i);
                        }
                    }
                    Pressed => {
                        if !pressed_keys.contains(&keycode) {
                            pressed_keys.push(keycode);
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }

            Event::MainEventsCleared => {
                let gl_window = context.window();

                winit_platform
                    .prepare_frame(imgui.io_mut(), &gl_window)
                    .expect("Failed to prepare frame");

                gl_window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Compute time passed since the previous frame and since the start of the program
                let now = std::time::Instant::now();
                let _elapsed = now.duration_since(first_frame_time).as_secs_f32();
                let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
                previous_frame_time = now;

                imgui
                    .io_mut()
                    .update_delta_time(std::time::Duration::from_secs_f32(delta_time));

                // Handle resize events
                if window_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(window_size.0, window_size.1));

                    winit_platform.attach_window(
                        imgui.io_mut(),
                        &context.window(),
                        imgui_winit_support::HiDpiMode::Default,
                    );

                    window_aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
                    (window_size).2 = false;
                    println!("Resized");
                    unsafe {
                        gl::Viewport(0, 0, window_size.0 as i32, window_size.1 as i32);
                    }
                }

                // Handle keyboard input
                for key in pressed_keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html
                        VirtualKeyCode::D => {
                            //print
                            cam_pos += move_speed
                                * delta_time
                                * glm::normalize(&glm::cross(&cam_front, &cam_up));
                        }
                        VirtualKeyCode::A => {
                            cam_pos -= move_speed
                                * delta_time
                                * glm::normalize(&glm::cross(&cam_front, &cam_up));
                        }

                        VirtualKeyCode::Space => {
                            cam_pos += move_speed * delta_time * cam_up;
                        }
                        VirtualKeyCode::LShift => {
                            cam_pos -= move_speed * delta_time * cam_up;
                        }

                        VirtualKeyCode::W => {
                            cam_pos += move_speed * delta_time * cam_front;
                        }
                        VirtualKeyCode::S => {
                            cam_pos -= move_speed * delta_time * cam_front;
                        }

                        VirtualKeyCode::Up => {
                            pitch += delta_time * cam_speed;
                        }
                        VirtualKeyCode::Down => {
                            pitch -= delta_time * cam_speed;
                        }

                        VirtualKeyCode::Left => {
                            yaw -= delta_time * cam_speed;
                        }
                        VirtualKeyCode::Right => {
                            yaw += delta_time * cam_speed;
                        }

                        // default handler:
                        _ => {}
                    }
                }

                unsafe {
                    let mut transformation_matrix: glm::Mat4 = glm::identity();
                    let mut view_matrix: glm::Mat4 = glm::identity();

                    cam_dir.x = yaw.to_radians().cos() * pitch.to_radians().cos();
                    cam_dir.y = pitch.to_radians().sin();
                    cam_dir.z = yaw.to_radians().sin() * pitch.to_radians().cos();

                    cam_front = glm::normalize(&cam_dir);

                    let look_at: glm::Mat4 =
                        glm::look_at(&cam_pos, &(cam_pos + cam_front), &cam_up);

                    view_matrix = look_at * view_matrix;

                    let projection_matrix: glm::Mat4 =
                        glm::perspective(window_aspect_ratio, glm::half_pi(), 1.0, VIEW_DISTANCE);

                    transformation_matrix = projection_matrix * view_matrix * transformation_matrix;

                    // Clear the color and depth buffers
                    gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                    let window = context.window();
                    let ui = imgui.frame();

                    let mut new_mesh_settings = mesh_settings.clone();
                    let mut new_noise_map_settings = noise_map_settings.clone();
                    let mut new_material_settings = material_settings.clone();

                    ui.window("Settings")
                        .size([300.0, 800.0], Condition::FirstUseEver)
                        .build(|| {
                            ui.text(format!("FPS: {}", (1.0 / delta_time).ceil()));
                            ui.separator();

                            ui.text("Terrain Settings");
                            new_noise_map_settings.render(ui);

                            ui.text("Mesh Settings");
                            new_mesh_settings.render(ui);

                            ui.separator();
                            ui.text("Lighting");
                            point_light_settings.render(ui);

                            ui.separator();
                            ui.text("Material Settings");
                            for material_setting in &mut new_material_settings {
                                material_setting.render(ui);
                            }
                        });

                    let mut should_rebuild = false;

                    if new_material_settings != material_settings {
                        // Update materials to match new settings
                        let mut new_materials = vec![];
                        for material_setting in &new_material_settings {
                            new_materials.push(Material::new(&MaterialSettings {
                                name: material_setting.name.clone(),
                                ambient: material_setting.ambient,
                                diffuse: material_setting.diffuse,
                                specular: material_setting.specular,
                                shininess: material_setting.shininess,
                                height_limit: material_setting.height_limit,
                            }));
                        }

                        //Sort materials
                        new_materials.sort_by(|a, b| {
                            a.height_limit
                                .partial_cmp(&b.height_limit)
                                .unwrap_or(Ordering::Equal)
                        });

                        material_settings = new_material_settings;
                        materials = new_materials;
                        should_rebuild = true;
                    }

                    if new_noise_map_settings != noise_map_settings {
                        noise_map_settings = new_noise_map_settings;
                        should_rebuild = true;
                    }

                    if new_mesh_settings != mesh_settings {
                        mesh_settings = new_mesh_settings;

                        should_rebuild = true;
                    }

                    if should_rebuild {
                        chunk_container.update_current_visible_chunks(
                            &materials,
                            &noise_map_settings,
                            &mesh_settings,
                        );
                    }

                    // thread::spawn(move || {
                    //     if let Ok(mut chunk_container) = arc_chunk_container_clone.lock() {
                    //     }
                    // });

                    chunk_container.generate_visible_chunks(cam_pos, &materials);
                    let scene: Vec<SceneNode> =
                        chunk_container.generate_scene(shape_shader.program_id);

                    draw_scene(
                        &scene,
                        &transformation_matrix,
                        &point_light_settings.get_point_light(),
                        &cam_pos,
                    );
                    winit_platform.prepare_render(&ui, &window);
                    renderer.render(&mut imgui);
                }

                // Display the new color buffer on the display
                context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                winit_platform.handle_event(imgui.io_mut(), context.window(), &event);
            }
        }
    });
}
