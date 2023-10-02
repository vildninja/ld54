use std::f32::consts::PI;
use macroquad::input::KeyCode::W;
use macroquad::prelude::*;

#[macroquad::main("A Tiny Corner of the Universe")]
async fn main() {

    let bg_tex: Texture2D = load_texture("bg.png").await.unwrap();
    let title_tex: Texture2D = load_texture("title.png").await.unwrap();
    let rot_tex: Texture2D = load_texture("rotate.png").await.unwrap();
    let up_tex: Texture2D = load_texture("up.png").await.unwrap();

    let rocket_tex: Texture2D = load_texture("rocket.png").await.unwrap();
    let flame_tex: Texture2D = load_texture("flame.png").await.unwrap();

    let planet_tex = vec![
        load_texture("planet_01.png").await.unwrap(),
        load_texture("planet_02.png").await.unwrap(),
        load_texture("planet_03.png").await.unwrap(),
        load_texture("planet_04.png").await.unwrap(),
        load_texture("planet_05.png").await.unwrap(),
        load_texture("planet_06.png").await.unwrap(),
        load_texture("planet_07.png").await.unwrap(),
        load_texture("planet_08.png").await.unwrap(),
        load_texture("planet_09.png").await.unwrap(),
    ];

    let g_const = 3.0f32;
    let rot_input_speed = PI;
    let thrust_input_accel = Vec2::new(0., -300.);

    let rocket_radius = 15f32;
    let rocket_points = [
        Vec2::new(0., -32.),
        Vec2::new(-22., 32.),
        Vec2::new(22., 32.),
    ];

    loop {
        let mut rocket = Rocket {
            texture: rocket_tex.clone(),
            position: Vec2::new(120., 120.),
            rotation: PI * 3. / 4.,
            mass: 100.,
            velocity: Vec2::new(-60., -60.),
        };


        let mut planets = Vec::new();

        planets.push(Planet {
            position: Vec2::new(860., 800.),
            rotation: rand::gen_range(-PI, PI),
            radius: 200.,
            atmosphere: 350.,
            speed: 0.2,
            mass: 10000.,
            texture: planet_tex[0].clone(),
        });
        planets.push(Planet {
            position: Vec2::new(800., 270.),
            rotation: rand::gen_range(-PI, PI),
            radius: 200.,
            atmosphere: 350.,
            speed: 0.2,
            mass: 10000.,
            texture: planet_tex[1].clone(),
        });
        planets.push(Planet {
            position: Vec2::new(280., 470.),
            rotation: rand::gen_range(-PI, PI),
            radius: 180.,
            atmosphere: 320.,
            speed: 0.2,
            mass: 10000.,
            texture: planet_tex[2].clone(),
        });
        planets.push(Planet {
            position: Vec2::new(1300., 310.),
            rotation: rand::gen_range(-PI, PI),
            radius: 220.,
            atmosphere: 370.,
            speed: 0.3,
            mass: 10000.,
            texture: planet_tex[3].clone(),
        });
        planets.push(Planet {
            position: Vec2::new(1100., 1300.),
            rotation: rand::gen_range(-PI, PI),
            radius: 160.,
            atmosphere: 260.,
            speed: -0.1,
            mass: 10000.,
            texture: planet_tex[4].clone(),
        });

        planets.push(Planet {
            position: Vec2::new(370., 1100.),
            rotation: rand::gen_range(-PI, PI),
            radius: 250.,
            atmosphere: 400.,
            speed: -0.4,
            mass: 20000.,
            texture: planet_tex[5].clone(),
        });
        planets.push(Planet {
            position: Vec2::new(1550., 850.),
            rotation: rand::gen_range(-PI, PI),
            radius: 190.,
            atmosphere: 370.,
            speed: -0.1,
            mass: 10000.,
            texture: planet_tex[6].clone(),
        });
        planets.push(Planet {
            position: Vec2::new(370., 1700.),
            rotation: rand::gen_range(-PI, PI),
            radius: 140.,
            atmosphere: 280.,
            speed: -0.1,
            mass: 10000.,
            texture: planet_tex[7].clone(),
        });
        planets.push(Planet {
            position: Vec2::new(1840., 230.),
            rotation: rand::gen_range(-PI, PI),
            radius: 130.,
            atmosphere: 210.,
            speed: -0.3,
            mass: 10000.,
            texture: planet_tex[8].clone(),
        });

        let game_time = get_time();

        let get_game_time = || get_time() - game_time;

        let mut is_alive = true;
        while is_alive {

            let mut input_left = is_key_down(KeyCode::A) ||
                is_key_down(KeyCode::Left);
            let mut input_right = is_key_down(KeyCode::D) ||
                is_key_down(KeyCode::Right);
            let mut input_thrust = is_key_down(KeyCode::W) ||
                is_key_down(KeyCode::Up) ||
                is_key_down(KeyCode::Space);

            if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                let mouse_pos = mouse_position_local();
                if mouse_pos.y < 0. {
                    input_thrust = true;
                }
                else if mouse_pos.x < -0.33 {
                    input_left = true;
                }
                else if mouse_pos.x > 0.33 {
                    input_right = true;
                }
                else {
                    input_thrust = true;
                }
            }

            for touch in touches_local() {
                if touch.position.y < 0. {
                    input_thrust = true;
                }
                else if touch.position.x < -0.33 {
                    input_left = true;
                }
                else if touch.position.x > 0.33 {
                    input_right = true;
                }
                else {
                    input_thrust = true;
                }
            }

            if get_game_time() < 1. {
                input_thrust = false;
            }

            let mut rot_speed: f32 = rot_input_speed * (f32::from(input_right) - f32::from(input_left));

            let dt = get_frame_time().clamp(0., 0.2);

            let mut acceleration = Vec2::default();


            for planet in &mut planets {
                let r_sq = planet.radius * planet.radius;

                let down = planet.position - rocket.position;
                let dist = down.length();
                let down_norm = down / dist;

                let dist_sq = (dist * dist).max(r_sq);
                let g_force = g_const * rocket.mass * planet.mass / dist_sq;

                planet.rotation = wrap_rotation(planet.rotation + planet.speed * dt);

                if dist < planet.atmosphere {
                    let full_rot = planet.radius + 50.;
                    let factor = 1. - (dist - full_rot) / (planet.atmosphere - full_rot);
                    rot_speed += factor.clamp(0., 1.) * planet.speed;
                }

                // dbg!(dist_sq);
                // dbg!(g_force);

                acceleration += down_norm * g_force;
            }

            rocket.rotation = wrap_rotation(rocket.rotation + rot_speed * dt);
            let rotator = Vec2::new(rocket.rotation.cos(), rocket.rotation.sin());

            if input_thrust {
                acceleration += rotator.rotate(thrust_input_accel);
            }

            let rocket_world_points =
                [
                    rocket.position + rotator.rotate(rocket_points[0]),
                    rocket.position + rotator.rotate(rocket_points[1]),
                    rocket.position + rotator.rotate(rocket_points[2]),
                ];

            rocket.velocity += acceleration * dt;
            rocket.position += rocket.velocity * dt;
            // dbg!(rocket.position);

            // death to world bounds
            if rocket.position.min_element() < -50. {
                is_alive = false;
            }
            else if rocket.position.length_squared() > square(2060.) {
                is_alive = false;
            }

            let aspect = screen_width() / screen_height();
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
            let title_height = screen_width() * (120. / 800.);
            draw_texture_ex(&title_tex, 0., screen_height() - title_height, GOLD,
                            DrawTextureParams {
                            dest_size: Some(Vec2::new(screen_width(), title_height)),
                            ..DrawTextureParams::default()
            });

            if get_game_time() < 10. {
                let alpha = ((10. - get_game_time()) / 10.) as f32;
                let color = Color::new(
                    GOLD.r * alpha + sky_color.r * (1. - alpha),
                    GOLD.g * alpha + sky_color.g * (1. - alpha),
                    GOLD.b * alpha + sky_color.b * (1. - alpha),
                    1.);

                draw_texture_ex(&rot_tex, 0., screen_height() / 2., color,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(screen_width() / 3., screen_width() * (100. / 170.) / 4.)),
                                    ..DrawTextureParams::default()
                                });
                draw_texture_ex(&rot_tex, screen_width() * 3. / 4., screen_height() / 2., color,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(screen_width() / 4., screen_width() * (100. / 170.) / 4.)),
                                    flip_x: true,
                                    ..DrawTextureParams::default()
                                });
                draw_texture_ex(&up_tex, (screen_width() - screen_height() / 10.) / 2., screen_height() / 2., color,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(screen_height() / 10., screen_height() / 3.)),
                                    ..DrawTextureParams::default()
                                });
            }

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
                                planet.rotation * 57.2957795, 2., WHITE);
            }

            for planet in &planets {
                let dist_sq = planet.position.distance_squared(rocket.position);

                if dist_sq < square(planet.atmosphere + 50.) {
                    let radius_sq = square(planet.radius);

                    if dist_sq < square(planet.radius + rocket_radius) {
                        // body hit
                        is_alive = false;
                    }
                    // else if rocket_world_points[0].distance_squared(planet.position) < radius_sq {
                    //     // rocket tip hit
                    //     is_alive = false;
                    // }
                }


                draw_texture_ex(&planet.texture,
                                planet.position.x - planet.radius,
                                planet.position.y - planet.radius,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(planet.radius * 2.02, planet.radius * 2.02)),
                                    rotation: planet.rotation,
                                    ..DrawTextureParams::default()
                                });
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

            // draw_circle_lines(rocket.position.x, rocket.position.y,
            //                   rocket_radius, 2., RED);
            //
            // draw_line(rocket_world_points[0].x, rocket_world_points[0].y,
            //           rocket_world_points[1].x, rocket_world_points[1].y,
            //           2., YELLOW);
            // draw_line(rocket_world_points[0].x, rocket_world_points[0].y,
            //           rocket_world_points[2].x, rocket_world_points[2].y,
            //           2., YELLOW);

            next_frame().await
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
}