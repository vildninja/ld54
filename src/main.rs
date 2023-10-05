use std::f32::consts::PI;
use std::ops::AddAssign;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

#[macroquad::main("A Tiny Corner of the Universe")]
async fn main() {

    let bg_tex: Texture2D = load_texture("bg.png").await.unwrap();
    let left_tex: Texture2D = load_texture("left.png").await.unwrap();
    let right_tex: Texture2D = load_texture("right.png").await.unwrap();
    let up_tex: Texture2D = load_texture("up.png").await.unwrap();

    let rocket_tex: Texture2D = load_texture("rocket.png").await.unwrap();
    let flame_tex: Texture2D = load_texture("flame.png").await.unwrap();

    let planet_tex = [
        load_texture("planet_01.png").await.unwrap(),
        load_texture("planet_02.png").await.unwrap(),
        load_texture("planet_03.png").await.unwrap(),
        load_texture("planet_04.png").await.unwrap(),
        load_texture("planet_05.png").await.unwrap(),
        load_texture("planet_06.png").await.unwrap(),
        load_texture("planet_07.png").await.unwrap(),
        load_texture("planet_08.png").await.unwrap(),
        load_texture("planet_09.png").await.unwrap(),
        load_texture("planet_10.png").await.unwrap(),
    ];

    simulate_mouse_with_touch(false);
    let mut mouse_input = true;
    let mut is_fullscreen = false;

    let mut k: Konst = include!("../Konst.txt");

    #[cfg(debug_assertions)]
    {
        mouse_input = false;
    }

    loop {
        let mut rocket = Rocket {
            texture: rocket_tex.clone(),
            position: Vec2::new(120., 120.),
            rotation: PI * 3. / 4.,
            mass: 100.,
            velocity: Vec2::new(-60., -60.),
            spin: - PI / 4.,
        };


        let mut planets = [
            Planet {
                position: Vec2::new(910., 800.),
                rotation: rand::gen_range(-PI, PI),
                radius: 200.,
                atmosphere: 350.,
                speed: 0.2,
                mass: 10000.,
                texture: planet_tex[0].clone(), // lake
            },
            Planet {
                position: Vec2::new(750., 270.),
                rotation: rand::gen_range(-PI, PI),
                radius: 210.,
                atmosphere: 350.,
                speed: -0.22,
                mass: 10000.,
                texture: planet_tex[1].clone(), // red
            },
            Planet {
                position: Vec2::new(280., 470.),
                rotation: rand::gen_range(-PI, PI),
                radius: 180.,
                atmosphere: 320.,
                speed: 0.2,
                mass: 10000.,
                texture: planet_tex[2].clone(), // ice
            },
            Planet {
                position: Vec2::new(1350., 340.),
                rotation: rand::gen_range(-PI, PI),
                radius: 220.,
                atmosphere: 370.,
                speed: 0.35,
                mass: 10000.,
                texture: planet_tex[3].clone(), // island
            },
            Planet {
                position: Vec2::new(1550., 850.),
                rotation: rand::gen_range(-PI, PI),
                radius: 160.,
                atmosphere: 260.,
                speed: -0.1,
                mass: 10000.,
                texture: planet_tex[4].clone(), // no moon
            },
            Planet {
                position: Vec2::new(370., 1100.),
                rotation: rand::gen_range(-PI, PI),
                radius: 250.,
                atmosphere: 400.,
                speed: -0.4,
                mass: 20000.,
                texture: planet_tex[5].clone(), // dragon
            },
            Planet {
                position: Vec2::new(1100., 1300.),
                rotation: rand::gen_range(-PI, PI),
                radius: 190.,
                atmosphere: 370.,
                speed: -0.1,
                mass: 10000.,
                texture: planet_tex[6].clone(), // lava
            },
            Planet {
                position: Vec2::new(300., 1700.),
                rotation: rand::gen_range(-PI, PI),
                radius: 170.,
                atmosphere: 310.,
                speed: -0.3,
                mass: 10000.,
                texture: planet_tex[7].clone(), // river
            },
            Planet {
                position: Vec2::new(1780., 200.),
                rotation: rand::gen_range(-PI, PI),
                radius: 130.,
                atmosphere: 210.,
                speed: -0.4,
                mass: 15000.,
                texture: planet_tex[8].clone(), // bloom
            },
            Planet {
                position: Vec2::new(690., 1540.),
                rotation: rand::gen_range(-PI, PI),
                radius: 110.,
                atmosphere: 240.,
                speed: 0.2,
                mass: 15000.,
                texture: planet_tex[9].clone(), // sub
            },
        ];

        let game_time = get_time();

        let get_game_time = || get_time() - game_time;

        let mut is_alive = true;
        while is_alive {

            let screen_dim = Vec2::new(screen_width(), screen_height());
            let min_dim = screen_dim.min_element();
            let third_dim = min_dim / 3.;

            let mut input_left = is_key_down(KeyCode::A) ||
                is_key_down(KeyCode::Left);
            let mut input_right = is_key_down(KeyCode::D) ||
                is_key_down(KeyCode::Right);
            let mut input_thrust = is_key_down(KeyCode::W) ||
                is_key_down(KeyCode::Up) ||
                is_key_down(KeyCode::Space);

            if is_key_pressed(KeyCode::F) ||
                (mouse_input && is_mouse_button_pressed(MouseButton::Left) && mouse_position().1 < third_dim) {
                set_fullscreen(!is_fullscreen);
                is_fullscreen = !is_fullscreen;
            }
            if is_key_pressed(KeyCode::Escape) {

                #[cfg(not(target_arch = "wasm32"))]
                if !is_fullscreen {
                    return;
                }

                set_fullscreen(false);
                is_fullscreen = false;
            }

            if mouse_input {
                if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                    let mouse_pos = mouse_position();
                    if mouse_pos.1 < third_dim {
                        // fullscreen?
                        // input_thrust = true;
                    } else if mouse_pos.0 > screen_dim.x - third_dim {
                        input_right = true;
                    } else if mouse_pos.0 > screen_dim.x - third_dim * 2. {
                        input_left = true;
                    } else {
                        input_thrust = true;
                    }
                }
            }

            for touch in touches() {
                if touch.position.y < third_dim {
                    input_thrust = true;
                }
                else if touch.position.x > screen_dim.x - third_dim {
                    input_right = true;
                }
                else if touch.position.x >  screen_dim.x - third_dim * 2. {
                    input_left = true;
                }
                else {
                    input_thrust = true;
                }
            }

            if get_game_time() < 1. {
                input_thrust = false;
            }

            #[cfg(debug_assertions)]
            root_ui().window(hash!(),
                             Vec2::new(0., third_dim),
                             Vec2::new(200., 200.),
                             |ui| {
                                 ui.checkbox(hash!(), "mouse input", &mut mouse_input);
                                 ui.drag(hash!(), "g const", (0.001, 100.), &mut k.g_const);
                                 ui.drag(hash!(), "spin input", (1., 200.), &mut k.spin_input_torque);
                                 ui.drag(hash!(), "spin drag", (0.001, 50.), &mut k.spin_drag);
                                 ui.drag(hash!(), "surface brake", (0.001, 50.), &mut k.surface_brake);
                                 ui.drag(hash!(), "thrust input", (100., 1000.), &mut k.thrust_input_accel);

                                 #[cfg(not(target_arch = "wasm32"))]
                                 if ui.button(None, "Save") {
                                     use std::fs::File;
                                     use std::io::Write;
                                     File::create("Konst.txt").unwrap().
                                         write_all(format!("{k:?}").as_bytes()).unwrap();
                                 }
                             });

            let mut torque: f32 = k.spin_input_torque * (f32::from(input_right) - f32::from(input_left));

            let dt = get_frame_time().clamp(0., 0.2);

            let mut acceleration = Vec2::default();

            let mut base_spin = 0.;

            for planet in &mut planets {

                let down = planet.position - rocket.position;
                let dist = down.length();
                let down_norm = down / dist;

                let g_force = k.g_const * rocket.mass * planet.mass / square(dist);

                planet.rotation = wrap_rotation(planet.rotation + planet.speed * dt);

                if dist < planet.atmosphere {
                    let full_rot = planet.radius + 50.;
                    let factor = 1. - (dist - full_rot) / (planet.atmosphere - full_rot);
                    base_spin += factor.clamp(0., 1.) * planet.speed;
                }

                // dbg!(dist_sq);
                // dbg!(g_force);

                acceleration += down_norm * g_force;
            }

            rocket.spin += torque * dt;
            rocket.spin = lerp_exp(rocket.spin, base_spin, k.spin_drag, dt);

            rocket.rotation = wrap_rotation(rocket.rotation + rocket.spin * dt);
            let rotator = Vec2::new(rocket.rotation.cos(), rocket.rotation.sin());

            if input_thrust {
                acceleration += rotator.rotate(Vec2::new(0., -k.thrust_input_accel));
            }

            rocket.velocity += acceleration * dt;
            rocket.position += rocket.velocity * dt;
            // dbg!(rocket.position);

            let foot_position = [
                rocket.position + rotator.rotate(Vec2::new(-k.rocket_feet_width, k.rocket_feet_height)),
                rocket.position + rotator.rotate(Vec2::new(k.rocket_feet_width, k.rocket_feet_height)),
            ];

            let danger_radius = Vec2::new(k.rocket_feet_width, k.rocket_feet_height).length();
            for planet in &planets {
                let dist_sq = planet.position.distance_squared(rocket.position);

                if dist_sq < square(planet.radius + danger_radius) {
                    if dist_sq < square(planet.radius + k.rocket_radius) {
                        // body hit
                        is_alive = false;
                        break;
                    }


                    let mut resolve = None;

                    for foot in foot_position {
                        let on_ground = foot - planet.position;
                        let planet_dist = on_ground.length();
                        if planet_dist < planet.radius {
                            resolve.get_or_insert(Vec2::default()).
                                add_assign(on_ground * (planet.radius - planet_dist) / planet_dist);
                        }
                    }

                    if let Some(push) = resolve {

                        rocket.rotation += planet.speed * dt;


                        // only resolve half
                        rocket.position += push / 2.;
                        let negate_velocity = rocket.velocity.project_onto(push);
                        rocket.velocity -= negate_velocity;

                        let surface_velocity = (rocket.position - planet.position).perp() * planet.speed;
                        let old_surface_velocity = rocket.velocity.project_onto(surface_velocity);
                        rocket.velocity -= old_surface_velocity;
                        rocket.velocity += v_lerp_exp(old_surface_velocity, surface_velocity, k.surface_brake, dt);
                    }

                }
            }

            // death to world bounds
            if rocket.position.min_element() < -50. {
                is_alive = false;
            }
            else if rocket.position.length_squared() > square(2060.) {
                is_alive = false;
            }

            let aspect = screen_dim.x / screen_height();
            let zoom_amount = 1. / 600.;
            let zoom = if aspect > 1. {
                Vec2::new(zoom_amount / aspect, zoom_amount)
            }
            else {
                Vec2::new(zoom_amount, zoom_amount * aspect)
            };

            set_default_camera();
            let sky_color = Color::new(0.05, 0.0, 0.15, 1.00);
            clear_background(sky_color);

            let arrow_size = min_dim / 6.;
            let half_arrow_size = arrow_size / 2.;
            let arrow_y_pos = screen_height() - third_dim + half_arrow_size;

            draw_texture_ex(&left_tex, screen_dim.x - third_dim * 2. + half_arrow_size, arrow_y_pos, WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(arrow_size, arrow_size)),
                                ..DrawTextureParams::default()
                            });
            draw_texture_ex(&right_tex, screen_dim.x - third_dim + half_arrow_size, arrow_y_pos, WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(arrow_size, arrow_size)),
                                ..DrawTextureParams::default()
                            });
            draw_texture_ex(&up_tex, half_arrow_size, arrow_y_pos, WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(arrow_size, arrow_size)),
                                ..DrawTextureParams::default()
                            });

            let camera = Camera2D {
                zoom,
                rotation: rocket.rotation * -57.2957795,
                target: rocket.position,
                ..Camera2D::default()
            };
            set_camera(&camera);

            draw_texture_ex(&bg_tex, 0., 0., WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(2000., 2000.)),
                                ..DrawTextureParams::default()
                            });


            // draw atmosphere
            for planet in &planets {
                draw_poly_lines(planet.position.x, planet.position.y,
                                48, planet.atmosphere,
                                planet.rotation * 57.2957795, 5., Color::new(0.7, 0.7, 1.0, 0.1));
            }

            let mut sim = (rocket.position, rocket.velocity);
            let sim_dt = 0.05;
            let sim_max = 100;
            let mut min_dist:f32 = 1000.;
            'sim_path: for i in 0..sim_max {

                let mut sim_gravity = Vec2::default();
                for planet in &planets {
                    let down = planet.position - sim.0;
                    let dist = down.length();

                    min_dist = min_dist.min(dist - planet.radius);

                    if dist <= planet.radius + k.rocket_radius {
                        break 'sim_path;
                    }

                    let g_force = k.g_const * rocket.mass * planet.mass / square(dist);

                    let down_norm = down / dist;
                    sim_gravity += down_norm * g_force;
                }

                sim.1 += sim_gravity * sim_dt;
                let last = sim.0;
                sim.0 += sim.1 * sim_dt;


                let alpha = inv_lerp(sim_max as f32, sim_max as f32 / 2., i as f32).clamp(0.,1.);
                let safety = inv_lerp(k.rocket_radius, 50., min_dist).clamp(0.,1.);
                draw_line(last.x, last.y, sim.0.x, sim.0.y, 5.,
                          Color::new(
                              lerp(1.0, 0.5, safety),
                              lerp(0.4, 0.1, safety),
                              lerp(0.2, 0.0, safety),
                              0.1 * alpha));
            }

            for planet in &planets {
                let img_size = planet.radius * 2.05;
                draw_texture_ex(&planet.texture,
                                planet.position.x - img_size / 2.,
                                planet.position.y - img_size / 2.,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(img_size, img_size)),
                                    rotation: planet.rotation,
                                    ..DrawTextureParams::default()
                                });

                // draw_poly_lines(planet.position.x, planet.position.y,
                //                 48, planet.radius,
                //                 planet.rotation * 57.2957795, 1., RED);
            }

            draw_texture_ex(&rocket.texture, rocket.position.x - 25., rocket.position.y - 30., WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(50., 65.)),
                                pivot: Some(rocket.position),
                                rotation: rocket.rotation,
                                ..DrawTextureParams::default()
                            });
            if input_thrust {
                draw_texture_ex(&flame_tex, rocket.position.x - 15., rocket.position.y + 25., WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(30., 40.)),
                                    pivot: Some(rocket.position),
                                    rotation: rocket.rotation,
                                    flip_x: ((get_time() * 25.) as i32) & 2 == 0,
                                    ..DrawTextureParams::default()
                                });
            }

            #[cfg(debug_assertions)]
            {
                // draw_circle_lines(rocket.position.x, rocket.position.y,
                //                   rocket_radius, 2., RED);
                //
                // draw_line(rocket_world_points[0].x, rocket_world_points[0].y,
                //           rocket_world_points[1].x, rocket_world_points[1].y,
                //           2., YELLOW);
                // draw_line(rocket_world_points[0].x, rocket_world_points[0].y,
                //           rocket_world_points[2].x, rocket_world_points[2].y,
                //           2., YELLOW);

                draw_line(100., 0., 500., 0., 2., RED);
                draw_line(0., 100., 0., 500., 2., GREEN);
                draw_line(1000., 0., 1000., 500., 2., BLACK);
                draw_line(0., 1000., 500., 1000., 2., BLACK);
            }

            next_frame().await
        }
    }
}

#[derive(Debug)]
struct Konst
{
    g_const: f32,
    spin_input_torque: f32,
    spin_drag: f32,
    surface_brake: f32,
    thrust_input_accel: f32,
    rocket_radius: f32,
    rocket_feet_height: f32,
    rocket_feet_width: f32,
}

impl Default for Konst {
    fn default() -> Self {
        Konst{
            g_const: 3.0,
            spin_input_torque: PI * 10.,
            spin_drag: 0.5,
            surface_brake: 5.,
            thrust_input_accel: 300.,
            rocket_radius: 15.,
            rocket_feet_height: 32.,
            rocket_feet_width: 22.,
        }
    }
}

fn wrap_rotation(rotation: f32) -> f32
{
    if rotation > 2. * PI { rotation - 2. * PI }
    else if rotation < -2. * PI { rotation + 2. * PI }
    else { rotation }
}

fn square(base: f32) -> f32
{
    base * base
}

fn lerp(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t.clamp(0., 1.)
}

fn lerp_exp(value: f32, target: f32, speed: f32, dt: f32) -> f32 {
    lerp(target, value, (-speed * dt).exp2())
}

fn inv_lerp(zero: f32, one: f32, val: f32) -> f32 {
    (val - zero) / (one - zero)
}

fn v_lerp(from: Vec2, to: Vec2, t: f32) -> Vec2 {
    from + (to - from) * t.clamp(0., 1.)
}

fn v_lerp_exp(value: Vec2, target: Vec2, speed: f32, dt: f32) -> Vec2 {
    v_lerp(target, value, (-speed * dt).exp2())
}

struct Sprite
{
    texture: Texture2D,
    size: Vec2,
    pivot: Vec2,
}

struct Planet
{
    texture: Texture2D,
    position: Vec2,
    radius: f32,
    atmosphere: f32,
    mass: f32,
    rotation: f32,
    speed: f32,
}

struct House
{
    texture: Texture2D,
    position: f32, // radial position on planet
    height: f32,
    width: f32,
}

struct Rocket
{
    texture: Texture2D,
    position: Vec2,
    rotation: f32,
    mass: f32,
    velocity: Vec2,
    spin: f32,
}
