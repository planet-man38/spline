use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Bezier {
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
}

#[macroquad::main("spline")]
async fn main() {
    let mut curve = Bezier{
        p0: Vec2{x: 100.0, y: 200.0},
        p1: Vec2{x: 500.0, y: 100.0},
        p2: Vec2{x: 600.0, y: 400.0},
        p3: Vec2{x: 500.0, y: 500.0},
    };

    let resolution: u32 = 50;

    let mut current_grab: i8 = -1;
    let mut current_select: i8 = -1;
    let mut current_curve: i8 = 1;
    let mut curves:Vec<Bezier> = vec![];

    loop { //MAIN LOOP
        if is_key_pressed(KeyCode::Right) {
            current_curve += 1;
        }
        if is_key_pressed(KeyCode::Left) {
            current_curve += -1;
        }

        clear_background(BLACK);

        
        curve_bollocks(&mut curve, &mut current_grab, &mut current_select, resolution, current_curve, 0);

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

fn check_pressed(curve: &mut Bezier, grab: &mut i8, select: &mut i8) {
    if is_mouse_button_pressed(MouseButton::Left) {
        check_closest(7.0, &[curve.p0, curve.p1, curve.p2, curve.p3], grab);
    }
    if is_mouse_button_down(MouseButton::Left) {
        if *grab >= 0 {
            let points = [&mut curve.p0, &mut curve.p1, &mut curve.p2, &mut curve.p3];
            *points[*grab as usize] = Vec2{x: mouse_position().0, y: mouse_position().1}; //moves both curves for some reason
            }
    } else {
        if check_closest(7.0, &[curve.p0, curve.p1, curve.p2, curve.p3], select) == -1 {
            *select = -1;
        }
    }
    if is_mouse_button_released(MouseButton::Left) {
        *grab = -1;
    }

    if *select != -1 {
        let points = [curve.p0, curve.p1, curve.p2, curve.p3];
        draw_point(points[*select as usize], 7.0, WHITE);
    }
}

fn curve_bollocks(curve: &mut Bezier, grab: &mut i8, select: &mut i8, resolution: u32, edited_index: i8, index: i8) {
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
                sampled_points.push(sample_bezier(i as f32 / maxi as f32, *curve));
                
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
                draw_line(curve.p0.x, curve.p0.y, curve.p1.x, curve.p1.y, 0.5, WHITE);
                draw_line(curve.p1.x, curve.p1.y, curve.p2.x, curve.p2.y, 0.5, WHITE);
                draw_line(curve.p2.x, curve.p2.y, curve.p3.x, curve.p3.y, 0.5, WHITE);

                check_pressed(curve, grab, select);
                draw_point(curve.p0, 5.0, RED);
                draw_point(curve.p1, 5.0, GREEN);
                draw_point(curve.p2, 5.0, GREEN);
                draw_point(curve.p3, 5.0, RED);
            }
}
