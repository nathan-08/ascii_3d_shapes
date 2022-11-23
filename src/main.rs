use ncurses::*;
use std::{thread, time};

fn rotate_yz(a: f32, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    (x, y*a.cos() - z*a.sin(), y*a.sin() + z*a.cos())
}
fn rotate_xy(a: f32, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    (x*a.cos() - y*a.sin(), x*a.sin() + y*a.cos(), z)
}
fn rotate_xz(a: f32, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    (x*a.cos() + z*a.sin(), y, -x*a.sin() + z*a.cos())
}

fn do_thing(cx: f32,
            cy: f32,
            angle: f32,
            points: &[(f32,f32,f32); 3],
            st: &'static str
    ) -> (f32, Vec<(f32,f32,f32)>, &'static str) { // f: A -> () ~ prop: p -> T
    let mut coords: [(f32, f32, f32); 3] = [(0.0, 0.0, 0.0); 3];
    for i in 0..points.len() {
        let point = points[i];
        let mut xx = point.0;
        let mut yy = point.1;
        let mut zz = point.2;
        // <xz, xy>
        //(xx, yy, zz) = rotate_xz(angle, xx, yy, zz);
        (xx, yy, zz) = rotate_yz(angle, xx, yy, zz);
        (xx, yy, zz) = rotate_xy(angle, xx, yy, zz);

        let distance = 0.2;
        let scale_factor = 8.0;
        let w = 1.0 / distance - zz;
        xx *= w;
        yy *= w;
        xx *= scale_factor;
        yy *= scale_factor;

        let xcoord = (xx + cx).round() as i32;
        let ycoord = ((yy + cx)/2.0).round() as i32;
        coords[i] = (xx + cx, (yy + cx)/2.0, zz);
    }
    let mut coords_vec = coords.to_vec();
    coords_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    (coords[0].2 + coords[1].2 + coords[2].2, coords_vec, st)
}

fn triangle_algorithm(coords: &Vec<(f32,f32,f32)>, st: &str) -> () {
    let (p1, p2, p3) = (coords[0], coords[1], coords[2]);
    let up = if p2.1 > p3.1 { p2 } else { p3 };
    let dn = if p2.1 < p3.1 { p2 } else { p3 };
    let upper_limit = std::cmp::max(up.1.round() as i32, p1.1.round() as i32);
    let lower_limit = std::cmp::min(dn.1.round() as i32, p1.1.round() as i32);

    for x in p1.0.round() as i32..=p2.0.round() as i32 {
        // determine upper and lower y's
        let slope1 = (up.1 - p1.1)/(up.0 - p1.0);
        let slope2 = (dn.1 - p1.1)/(dn.0 - p1.0);

        let greater_slope = if slope1 > slope2 { slope1 } else { slope2 };
        let lesser_slope = if slope1 < slope2  { slope1 } else { slope2 };
        let y_up = (p1.1 + greater_slope * (x as f32 - p1.0)).round() as i32; // p1 -> up
        let y_dn = (p1.1 + lesser_slope * (x as f32 - p1.0)).round() as i32; // p1 -> down
        for y in y_dn..=y_up {
            if y < lower_limit || y > upper_limit { continue; }
            mvaddstr(y, x, st);
        }
    }
    for x in p2.0.round() as i32..=p3.0.round() as i32 {
        //if p2 == up {
            let slope1 = (p3.1 - p2.1)/(p3.0 - p2.0);
            let slope2 = (p3.1 - p1.1)/(p3.0 - p1.0);
            let greater_slope = if slope1 > slope2 { slope1 } else { slope2 };
            let lesser_slope = if slope1 < slope2  { slope1 } else { slope2 };
            let y_up = (p3.1 + lesser_slope * (x as f32 - p3.0)).round() as i32; // p2 -> p3
            let y_dn = (p3.1 + greater_slope * (x as f32 - p3.0)).round() as i32; // p1 -> p3
            for y in y_dn..=y_up {
                if y < lower_limit || y > upper_limit { continue; }
                mvaddstr(y, x, st);
            }
        //} else {
            //let y_up = (p1.1 + (p3.1 - p1.1)/(p3.0 - p1.0) * (x as f32 - p1.0)).round() as i32; // p1 -> p3
            //let y_dn = (p2.1 + (p3.1 - p2.1)/(p3.0 - p2.0) * (x as f32 - p2.0)).round() as i32; // p2 -> p3
            //for y in y_dn..=y_up {
                //if y < lower_limit || y > upper_limit { continue; }
                //mvaddstr(y, x, st);
            //}
        //}
    }
}

fn main() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let cx: f32 = 35.0;
    let cy: f32 = cx * 0.5;
    let cz: f32 = 0.0;

    let points1: [(f32,f32,f32); 3] = [ // left, top, right
        (-0.5,  -1.0/(2.0*3.0_f32.sqrt()), 0.5), (0.0, 1.0/3.0_f32.sqrt(), 0.0),
        ( 0.5,  -1.0/(2.0*3.0_f32.sqrt()), 0.5),
    ];
    let points2: [(f32,f32,f32); 3] = [
        ( 0.0,  -1.0/(2.0*3.0_f32.sqrt()),-0.5), (0.0, 1.0/3.0_f32.sqrt(), 0.0),
        ( 0.5,  -1.0/(2.0*3.0_f32.sqrt()), 0.5),
    ];
    let points3: [(f32,f32,f32); 3] = [
        ( 0.0,  -1.0/(2.0*3.0_f32.sqrt()),-0.5), (0.0, 1.0/3.0_f32.sqrt(), 0.0),
        (-0.5,  -1.0/(2.0*3.0_f32.sqrt()), 0.5),
    ];
    let points4: [(f32,f32,f32); 3] = [
        (-0.5,  -1.0/(2.0*3.0_f32.sqrt()), 0.5), (0.5, -1.0/(2.0*3.0_f32.sqrt()),  0.5),
        ( 0.0,  -1.0/(2.0*3.0_f32.sqrt()),-0.5),
    ];

    let rot: f32 = (std::f32::consts::PI * 2.0) / 360.0;
    let mut angle: f32 = 0.0;

    loop {
        clear();
        let data1 = do_thing(cx, cy, angle, &points1, "*");
        let data2 = do_thing(cx, cy, angle, &points2, ";");
        let data3 = do_thing(cx, cy, angle, &points3, "~");
        let data4 = do_thing(cx, cy, angle, &points4, "'");
        let mut planes = vec![data1, data2, data3, data4];
        //let mut planes = vec![data3];
        planes.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        for (_, plane, st) in planes {
            triangle_algorithm(&plane, st);
        }
        refresh();
        angle += rot;
        thread::sleep(time::Duration::from_millis(30));
    }
    //getch();
    //endwin();
}
