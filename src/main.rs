use std::f32::consts::PI;
use macroquad::input::KeyCode::W;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

#[macroquad::main("A Tiny Corner of the Universe")]
async fn main() {

    let bg_tex: Texture2D = load_texture("bg.png").await.unwrap();
    let left_tex: Texture2D = load_texture("left.png").await.unwrap();
    let right_tex: Texture2D = load_texture("right.png").await.unwrap();
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
        Vec2::new(-22., 32.),
        Vec2::new(22., 32.),
    ];

    let mut is_fullscreen = false;

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
                (is_mouse_button_pressed(MouseButton::Left) && mouse_position().1 < third_dim) {
                set_fullscreen(is_fullscreen);
                is_fullscreen = !is_fullscreen;
            }

            if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                let mouse_pos = mouse_position();
                if mouse_pos.1 < third_dim {
                    // fullscreen?
                    // input_thrust = true;
                }
                else if mouse_pos.0 > screen_dim.x - third_dim {
                    input_right = true;
                }
                else if mouse_pos.0 > screen_dim.x - third_dim * 2. {
                    input_left = true;
                }
                else {
                    input_thrust = true;
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

            for planet in &planets {
                let dist_sq = planet.position.distance_squared(rocket.position);

                if dist_sq < square(planet.atmosphere + 50.) {
                    let radius_sq = square(planet.radius);

                    if dist_sq < square(planet.radius + rocket_radius) {
                        // body hit
                        is_alive = false;
                    }
                    else {
                        let left_foot = (planet.radius - rocket_world_points[0].distance(planet.position)).max(0.);
                        let right_foot = (planet.radius - rocket_world_points[1].distance(planet.position)).max(0.);
                    }
                }

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

            // draw_circle_lines(rocket.position.x, rocket.position.y,
            //                   rocket_radius, 2., RED);
            //
            // draw_line(rocket_world_points[0].x, rocket_world_points[0].y,
            //           rocket_world_points[1].x, rocket_world_points[1].y,
            //           2., YELLOW);
            // draw_line(rocket_world_points[0].x, rocket_world_points[0].y,
            //           rocket_world_points[2].x, rocket_world_points[2].y,
            //           2., YELLOW);

            // draw_line(100., 0., 500., 0., 2., RED);
            // draw_line(0., 100., 0., 500., 2., GREEN);
            // draw_line(1000., 0., 1000., 500., 2., BLACK);
            // draw_line(0., 1000., 500., 1000., 2., BLACK);

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