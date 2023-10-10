use std::f32::consts::PI;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::root_ui;


#[macroquad::main("A Tiny Corner of the Universe")]
async fn main() {

    let bg_tex: Texture2D = load_texture("res/bg.png").await.unwrap();
    let left_tex: Texture2D = load_texture("res/left.png").await.unwrap();
    let right_tex: Texture2D = load_texture("res/right.png").await.unwrap();
    let up_tex: Texture2D = load_texture("res/up.png").await.unwrap();

    let dir_indicator_tex = load_texture("res/dir.png").await.unwrap();

    let rocket_tex: Texture2D = load_texture("res/rocket.png").await.unwrap();
    let flame_tex: Texture2D = load_texture("res/flame.png").await.unwrap();

    let planet_tex = [
        load_texture("res/planet_01.png").await.unwrap(),
        load_texture("res/planet_02.png").await.unwrap(),
        load_texture("res/planet_03.png").await.unwrap(),
        load_texture("res/planet_04.png").await.unwrap(),
        load_texture("res/planet_05.png").await.unwrap(),
        load_texture("res/planet_06.png").await.unwrap(),
        load_texture("res/planet_07.png").await.unwrap(),
        load_texture("res/planet_08.png").await.unwrap(),
        load_texture("res/planet_09.png").await.unwrap(),
        load_texture("res/planet_10.png").await.unwrap(),
    ];


    let mut house_tex: Vec<Texture2D> = Vec::new();
    for number in  2..=22 {
        house_tex.push(load_texture(&format!("res/house_{number}.png")).await.unwrap());
    }

    simulate_mouse_with_touch(false);
    let mut mouse_input = true;
    let mut is_fullscreen = false;

    let mut k: Konst = include!("../Konst.txt");


    #[cfg(debug_assertions)]
    {
        mouse_input = false;
    }

    let mut last_death: Option<Vec2> = None;

    loop {
        let mut rocket = Rocket {
            texture: rocket_tex.clone(),
            position: Vec2::new(120., 120.),
            rotation: PI * 3. / 4.,
            mass: 100.,
            velocity: Vec2::new(-60., -60.),
            spin: 0.,
        };


        let mut planets = [
            Planet {
                position: Vec2::new(910., 800.),
                rotation: rand::gen_range(-PI, PI),
                radius: 200.,
                atmosphere: 350.,
                spin: 0.2,
                mass: 10000.,
                texture: planet_tex[0].clone(), // lake
            },
            Planet {
                position: Vec2::new(750., 270.),
                rotation: rand::gen_range(-PI, PI),
                radius: 210.,
                atmosphere: 350.,
                spin: -0.22,
                mass: 10000.,
                texture: planet_tex[1].clone(), // red
            },
            Planet {
                position: Vec2::new(280., 470.),
                rotation: rand::gen_range(-PI, PI),
                radius: 180.,
                atmosphere: 320.,
                spin: 0.2,
                mass: 10000.,
                texture: planet_tex[2].clone(), // ice
            },
            Planet {
                position: Vec2::new(1350., 340.),
                rotation: rand::gen_range(-PI, PI),
                radius: 220.,
                atmosphere: 370.,
                spin: 0.35,
                mass: 10000.,
                texture: planet_tex[3].clone(), // island
            },
            Planet {
                position: Vec2::new(1550., 850.),
                rotation: rand::gen_range(-PI, PI),
                radius: 160.,
                atmosphere: 260.,
                spin: -0.1,
                mass: 10000.,
                texture: planet_tex[4].clone(), // no moon
            },
            Planet {
                position: Vec2::new(370., 1100.),
                rotation: rand::gen_range(-PI, PI),
                radius: 250.,
                atmosphere: 400.,
                spin: -0.4,
                mass: 20000.,
                texture: planet_tex[5].clone(), // dragon
            },
            Planet {
                position: Vec2::new(1100., 1300.),
                rotation: rand::gen_range(-PI, PI),
                radius: 190.,
                atmosphere: 370.,
                spin: -0.1,
                mass: 10000.,
                texture: planet_tex[6].clone(), // lava
            },
            Planet {
                position: Vec2::new(300., 1700.),
                rotation: rand::gen_range(-PI, PI),
                radius: 170.,
                atmosphere: 310.,
                spin: -0.3,
                mass: 10000.,
                texture: planet_tex[7].clone(), // river
            },
            Planet {
                position: Vec2::new(1780., 200.),
                rotation: rand::gen_range(-PI, PI),
                radius: 130.,
                atmosphere: 210.,
                spin: -0.4,
                mass: 15000.,
                texture: planet_tex[8].clone(), // bloom
            },
            Planet {
                position: Vec2::new(690., 1540.),
                rotation: rand::gen_range(-PI, PI),
                radius: 110.,
                atmosphere: 240.,
                spin: 0.2,
                mass: 15000.,
                texture: planet_tex[9].clone(), // sub
            },
        ];

        let game_time = get_time();
        let mut air_time: f32 = 10.;
        let mut grounded = false;

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
                             Vec2::new(10., 10.),
                             Vec2::new(200., 300.),
                             |ui| {
                                 ui.checkbox(hash!(), "mouse input", &mut mouse_input);
                                 ui.drag(hash!(), "g const", (0.001, 100.), &mut k.g_const);
                                 ui.drag(hash!(), "spin input", (1., 200.), &mut k.spin_input_torque);
                                 ui.drag(hash!(), "max spin", (1., 100.), &mut k.max_spin);
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

                planet.rotation = wrap_rotation(planet.rotation + planet.spin * dt);

                if dist < planet.atmosphere {
                    let full_rot = planet.radius + 50.;
                    let factor = 1. - (dist - full_rot) / (planet.atmosphere - full_rot);
                    base_spin += factor.clamp(0., 1.) * planet.spin;
                }

                // dbg!(dist_sq);
                // dbg!(g_force);

                acceleration += down_norm * g_force;
            }

            rocket.spin += torque * dt;
            rocket.spin = lerp_exp(rocket.spin, base_spin, k.spin_drag, dt). //  * air_time.min(1.)
                clamp(-k.max_spin, k.max_spin);

            rocket.rotation = wrap_rotation(rocket.rotation + rocket.spin * dt);
            let rotator = Vec2::new(rocket.rotation.cos(), rocket.rotation.sin());

            if input_thrust {
                acceleration += rotator.rotate(Vec2::new(0., -k.thrust_input_accel));
            }

            rocket.velocity += acceleration * dt;
            rocket.position += rocket.velocity * dt;
            // dbg!(rocket.position);

            air_time += dt;

            let foot_offset = [
                rotator.rotate(Vec2::new(-k.rocket_feet_width, k.rocket_feet_height)),
                rotator.rotate(Vec2::new(k.rocket_feet_width, k.rocket_feet_height)),
            ];

            let foot_length = foot_offset[0].length();

            let mut collisions = Vec::new();

            for planet in &planets {
                let mut dist_sq = planet.position.distance_squared(rocket.position);

                if dist_sq < square(planet.radius + foot_length) {

                    let feet = foot_offset.map(|offset| (
                        offset,
                        rocket.position + offset,
                        rocket.velocity + offset.perp() * rocket.spin * dt,
                    ));

                    let mut impact_result: Option<(Vec2, Vec2, f32)> = None;
                    let mut num_impacts = 0;

                    for (offset, position, velocity) in feet {

                        let on_ground = position - planet.position;
                        let planet_dist = on_ground.length();
                        if planet_dist < planet.radius {

                            num_impacts += 1;
                            let ground_norm = on_ground / planet_dist;

                            let ground_offset = ground_norm * (planet.radius - planet_dist);
                            let ground_velocity = on_ground.perp() * planet.spin;
                            // let impact_vector = on_ground + ground_velocity;
                            // let impact_up = -velocity.project_onto_normalized(dbg!(ground_norm));
                            let ground_dot = ground_norm.dot(velocity).min(0.);
                            let impact_up_velocity = ground_norm * -dbg!(ground_dot);
                            let impact_ground_velocity = ground_velocity - velocity.project_onto_normalized(ground_norm.perp());
                            let impact_velocity = dbg!(impact_up_velocity) + dbg!(impact_ground_velocity);
                            let impact_spin = offset.perp_dot(impact_velocity);

                            collisions.push((position, impact_velocity));

                            impact_result = if let Some((cur_offset, cur_velocity, cur_spin)) = impact_result {
                                let combined_offset = cur_offset + ground_offset;
                                Some((
                                    if cur_offset.length_squared() > ground_offset.length_squared() {
                                        cur_offset.project_onto(combined_offset)
                                    }
                                    else {
                                        ground_offset.project_onto(combined_offset)
                                    },
                                    (cur_velocity + impact_velocity) / 2.,
                                    if cur_spin.is_sign_positive() == impact_spin.is_sign_positive() {
                                        cur_spin.abs().max(impact_spin.abs()).copysign(cur_spin)
                                    }
                                    else {
                                        cur_spin + impact_spin
                                    }
                                ))
                            }
                            else {
                                Some((ground_offset, impact_velocity, impact_spin))
                            };
                        }
                    }

                    if let Some((ground_offset, impact_velocity, impact_spin)) = impact_result {
                        rocket.position += ground_offset * 0.9;
                        rocket.velocity += impact_velocity * 0.9;

                        let spin_boost = if grounded || num_impacts != 1 { 1. } else { 5. };
                        let r_inertia = 4. / (PI * square(square(k.rocket_radius)));
                        rocket.spin += impact_spin * r_inertia * spin_boost;

                        // update dist_sq to prevent death from unresolved collisions
                        dist_sq = planet.position.distance_squared(rocket.position);
                        air_time = 0.;

                        let wanted_ground_speed = (rocket.position - planet.position).perp() * planet.spin;

                        if num_impacts == 2 &&
                            dbg!(rocket.spin - planet.spin).abs() < 0.1 &&
                            dbg!(rocket.velocity - wanted_ground_speed).length_squared() < 100. {
                            grounded = true;
                        }
                        // todo!("Implement grounded");

                    }

                    if dist_sq < square(planet.radius + k.rocket_radius) {
                        // body hit
                        is_alive = false;
                        break;
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
            let mut house_iter = house_tex.iter().cycle();

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

                let mut house_rot = 0.;
                for _ in 0..7 {
                    if let Some(next_house_tex) = house_iter.next() {
                        let tex_size = next_house_tex.size() * 0.75;
                        draw_texture_ex(next_house_tex,
                                        planet.position.x - tex_size.x / 2.,
                                        planet.position.y - planet.radius - tex_size.y + 7.,
                                        WHITE,
                                        DrawTextureParams {
                                            dest_size: Some(tex_size),
                                            rotation: planet.rotation + house_rot,
                                            pivot: Some(planet.position),
                                            ..DrawTextureParams::default()
                                        });
                        house_rot += PI * 2. * 1.618033988;
                    }
                }

                #[cfg(debug_assertions)]
                draw_poly_lines(planet.position.x, planet.position.y,
                                48, planet.radius,
                                planet.rotation * 57.2957795, 1., RED);
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
            draw_texture_ex(&dir_indicator_tex, rocket.position.x - 2., rocket.position.y + 15.,
                            Color::new(1., 1., 1., 0.6),
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(6., 8.)),
                                rotation: game_time as f32 * 10.,
                                ..DrawTextureParams::default()
                            });

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

                if let Some(death_pos) = &last_death {
                    draw_circle(death_pos.x, death_pos.y, k.rocket_radius, RED);
                }
                // if let Some(index) = killer_index {
                //     if let Some(killer) = &planets.get(index) {
                //         draw_circle_lines(killer.position.x, killer.position.y, killer.radius - 1., 2., RED);
                //     }
                // }

                draw_circle(rocket.position.x, rocket.position.y + 10., 5., SKYBLUE);
                draw_circle_lines(rocket.position.x, rocket.position.y, k.rocket_radius, 2., ORANGE);

                for (position, velocity) in &collisions {
                    draw_circle(position.x, position.y, 4., if grounded { GREEN } else { RED });
                    draw_line(position.x, position.y,
                              position.x + velocity.x, position.y + velocity.y,
                              1., BLUE);
                }
            }

            next_frame().await
        }
        last_death = Some(rocket.position);
    }
}

enum FlyMode {
    Grounded(i32),
    Flying(f32),
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
    max_spin: f32,
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
            max_spin: 10.,
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
    spin: f32,
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
