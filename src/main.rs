use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Rect, Point};

const BACKGROUND: Color = Color::RGB(18, 18, 18);
const FOREGROUND: Color = Color::RGB(255, 150, 150);
const DISTANCE: f64 = 3.0;

fn project([x, y, z]: [f64; 3], r: f64) -> [f64; 2] {
    return [x * r / z, y / z];
}

fn to_screen([x0, y0]: [f64; 2], w: f64, h: f64) -> [f64; 2] {
    let half_w = w * 0.5;
    let half_h = h * 0.5;
    let x = x0 * half_w + half_w;
    let y = y0 * half_h + half_h;
    [x, y]
}

fn translate([x0, y0, z0]: [f64; 3], [x1, y1, z1]: [f64; 3]) -> [f64; 3] {
    return [x0 + x1, y0 + y1, z0 + z1];
}

fn rotate_y([x0, y0, z0]: [f64; 3], theta: f64) -> [f64; 3] {
    let x1 = x0 * f64::cos(theta) + z0 * f64::sin(theta);
    let z1 = x0 * f64::sin(theta) - z0 * f64::cos(theta);
    return [x1, y0, z1];
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("4D", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut theta: f64 = 0.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }

        }

        canvas.set_draw_color(BACKGROUND);
        canvas.clear();
        let (w, h) = canvas.window().size();
        const low_range: f64 = -1.0;
        const high_range: f64 = 1.0;
        const SIZE: f64 = 10.0;
        const N: u32 = 5;
        const D: usize = 3;
        const ds: f64 = (high_range - low_range) / (N - 1) as f64;
        for ix in 0..N {
            for iy in 0..N {
                for iz in 0..N {
                    let p1 = [low_range + ix as f64 * ds,
                              low_range + iy as f64 * ds,
                              low_range + iz as f64 * ds];

                    for id in 0..D {
                        let p2 = {
                            let mut t = p1.clone();
                            t[id] += ds;
                            t
                        };

                        if (p2[id] <= high_range) {
                            let r = h as f64 / w as f64;
                            let ps1 = to_screen(
                                project(
                                    translate(
                                        rotate_y(p1, theta),
                                        [0.0, 0.0, DISTANCE]), r),
                                w as f64, h as f64);
                            let ps2 = to_screen(
                                project(
                                    translate(
                                        rotate_y(p2, theta),
                                        [0.0, 0.0, DISTANCE]), r),
                                w as f64, h as f64);

                            canvas.set_draw_color(FOREGROUND);
                            canvas.draw_line(Point::new(ps1[0] as i32, ps1[1] as i32),
                                             Point::new(ps2[0] as i32, ps2[1] as i32))?;
                        }
                    }
                }
            }
        }
        theta += 0.005;

        canvas.present();
    }

    Ok(())
}
