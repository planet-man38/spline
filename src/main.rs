use std::ops::{Add, Div};
use macroquad::prelude::*;
use macroquad::input::mouse_wheel;

#[derive(Clone, Copy)]
struct Bezier {
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
}

impl Add for Bezier {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            p0: self.p0 + other.p0,
            p1: self.p1 + other.p1,
            p2: self.p2 + other.p2,
            p3: self.p3 + other.p3
        }
    }
}
impl Add<Vec2> for Bezier {
    type Output = Self;

    fn add(self, other: Vec2) -> Self {
        Self {
            p0: self.p0 + other,
            p1: self.p1 + other,
            p2: self.p2 + other,
            p3: self.p3 + other,
        }
    }
}
impl Div<f32> for Bezier {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            p0: self.p0 / other,
            p1: self.p1 / other,
            p2: self.p2 / other,
            p3: self.p3 / other,
        }
    }
}

#[derive(Clone, Copy)]
struct Camera {
    position: Vec2,
    scale: f32
}

#[macroquad::main("spline")]
async fn main() {
    let resolution: u32 = 50;
    const SPEED: f32 = 20.0;

    let mut current_grab: i8 = -1;
    let mut current_select: i8 = -1;
    let mut current_curve: i8 = 0;
    let mut curves:Vec<Bezier> = vec![];
    let mut camera = Camera{
        position: Vec2 {x: 0.0, y: 0.0},
        scale: 1.0
    };

    loop { //MAIN LOOP
        if is_key_pressed(KeyCode::Right) {
            current_curve += 1;
        }
        if is_key_pressed(KeyCode::Left) {
            current_curve += -1;
        }

        cam_movement_check(&mut camera, SPEED);

        clear_background(BLACK);
        if is_key_pressed(KeyCode::Q) {
            curves.push(Bezier{
                p0: Vec2{x: 100.0, y: 200.0} + camera.position,
                p1: Vec2{x: 500.0, y: 100.0} + camera.position,
                p2: Vec2{x: 600.0, y: 400.0} + camera.position,
                p3: Vec2{x: 500.0, y: 500.0} + camera.position
            });
            current_curve = curves.len() as i8 - 1;
        }

        for i in 0..curves.len() {
            curve_bollocks(&mut curves[i], &mut current_grab, &mut current_select, resolution, current_curve, i as i8, camera);
        }

        next_frame().await
    }
}

fn draw_point(point: Vec2, r: f32, color: Color) {
    draw_circle(point.x, point.y, r, color);
}

fn dist(point1: Vec2, point2: Vec2) -> f32 {
    f32::sqrt((point1.x - point2.x).powf(2.0) + (point1.y - point2.y).powf(2.0))
}

fn sample_bezier(t: f32, points: Bezier) -> Vec2 {
        let result: Vec2;        
        result = (1.0 - t).powf(3.0) * points.p0 + 3.0 * (1.0 - t).powf(2.0) * t * points.p1 + 3.0 * (1.0 - t) * t * t * points.p2 + t * t * t * points.p3;
        result
}

fn check_closest(mindistance: f32, checked: &[Vec2], modified: &mut i8) -> i8 {
    let mut i: i8 = 0;
    let mut smallesti: i8 = -1;
    let mut dst: f32 = mindistance; //minimum distance the mouse can be before you can select point (idk how to phrase this)
    for p in checked {
        if dist(*p, Vec2{ x: mouse_position().0, y: mouse_position().1}) < dst {
            dst = dist(*p, Vec2{ x: mouse_position().0, y: mouse_position().1});
            *modified = i;
            if smallesti == -1 {
                smallesti = i;
            }
        }
        i += 1;
    }
    smallesti
}

fn check_pressed(curve: &mut Bezier, curve_visual: Bezier, grab: &mut i8, select: &mut i8, camera: Camera) {
    //check for which one should be moved upon click
    if is_mouse_button_pressed(MouseButton::Left) {
        check_closest(7.0, &[curve.p0 - camera.position, curve.p1 - camera.position, curve.p2 - camera.position, curve.p3 - camera.position], grab);
    }

    //move them yeah
    if is_mouse_button_down(MouseButton::Left) {
        if *grab >= 0 {
            let points = [&mut curve.p0, &mut curve.p1, &mut curve.p2, &mut curve.p3];
            *points[*grab as usize] = Vec2{x: mouse_position().0, y: mouse_position().1} + camera.position;
            }
    }
    //remove white slection thingy if mouse isn't hovering over anything
    else {
        if check_closest(7.0, &[curve.p0 - camera.position, curve.p1 - camera.position, curve.p2 - camera.position, curve.p3 - camera.position], select) == -1 {
            *select = -1;
        }
    }

    //deselect if released
    if is_mouse_button_released(MouseButton::Left) {
        *grab = -1;
    }

    //display white selection thingy when hovering
    if *select != -1 {
        let points = [curve_visual.p0, curve_visual.p1, curve_visual.p2, curve_visual.p3];
        draw_point(points[*select as usize], 7.0, WHITE);
    }
}

fn curve_bollocks(curve: &mut Bezier, grab: &mut i8, select: &mut i8, resolution: u32, edited_index: i8, index: i8, camera: Camera) {
            let drawing_curve: Bezier = (*curve + - camera.position) / camera.scale;
            let editing: bool;
            if edited_index == index {
                editing = true
            } else {
                editing = false
            }
            let mut sampled_points: Vec<Vec2> = vec![];
            let mut i = 0;
            let maxi = resolution;
            loop {
                sampled_points.push(sample_bezier(i as f32 / maxi as f32, drawing_curve));
                
                i += 1;
                if i > maxi {
                    break;
                }
            }

            i = 0;
            loop {
                if (i+1) <= maxi {
                    draw_line(sampled_points[i as usize].x, sampled_points[i as usize].y, sampled_points[i as usize+1].x, sampled_points[i as usize+1].y, 2.0, WHITE);
                }
    
                i += 1;
                if i > maxi {
                    break;
                }
            }
            
            if editing {
                draw_line(drawing_curve.p0.x, drawing_curve.p0.y, drawing_curve.p1.x, drawing_curve.p1.y, 0.5, WHITE);
                draw_line(drawing_curve.p1.x, drawing_curve.p1.y, drawing_curve.p2.x, drawing_curve.p2.y, 0.5, WHITE);
                draw_line(drawing_curve.p2.x, drawing_curve.p2.y, drawing_curve.p3.x, drawing_curve.p3.y, 0.5, WHITE);

                check_pressed(curve, drawing_curve,grab, select, camera);

                draw_point(drawing_curve.p0, 5.0, RED);
                draw_point(drawing_curve.p1, 5.0, GREEN);
                draw_point(drawing_curve.p2, 5.0, GREEN);
                draw_point(drawing_curve.p3, 5.0, RED);
            }
}

fn cam_movement_check(camera: &mut Camera, speed:f32) {
    let mut vel = Vec2{x: 0.0, y: 0.0};
    if is_key_down(KeyCode::D) {
        vel.x += speed * camera.scale;
    }
    if is_key_down(KeyCode::A) {
        vel.x += -speed * camera.scale;
    }
    if is_key_down(KeyCode::W) {
        vel.y += -speed * camera.scale;
    }
    if is_key_down(KeyCode::S) {
        vel.y += speed * camera.scale;
    }
    camera.position += vel;
    

    let scale_multiplier: f32 = (-mouse_wheel().1 + 360.0) / 360.0;
    camera.scale *= scale_multiplier;
    //camera.position *= scale_multiplier;
    let half_screen = Vec2{x: screen_width()/2.0, y: screen_height()/2.0};
    if scale_multiplier != 1.0 {
        camera.position += (Vec2{x: mouse_position().0, y: mouse_position().1} - half_screen) / (scale_multiplier - 1.0);
    }
}
