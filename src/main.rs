mod camera;
mod math;
mod primitive;
mod ray;
mod rgbcolor;
mod surface;
mod vec3;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{sync::mpsc::channel, thread};

use camera::Camera;
use ray::Ray;
use rgbcolor::RGBColor;
use surface::Surface;
use vec3::Vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: usize = 400;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLE_PER_PIXEL: u32 = 50;
const MAX_DEPTH: u32 = 50;

const VFOV: f64 = 20.0;
const EYE: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 10.0,
};
const LOOKAT: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const UP: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
const DIST_TO_FOCUS: f64 = 10.0;
const APERTURE: f64 = 0.1;

fn hit(ray: Ray) -> bool {
    let r = 1.0;
    let oc = ray.origin - Vec3::new(0.0, 0.0, 1.0);
    let a = ray.dir.length_sq();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_sq() - (r * r);
    let discriminant = (half_b * half_b) - (a * c);

    discriminant > 0.0
}

#[allow(clippy::many_single_char_names)]
fn get_background_color(x: usize, y: usize) -> RGBColor {
    let r = x as f64 / (IMG_WIDTH - 1) as f64;
    let g = y as f64 / (IMG_HEIGHT - 1) as f64;
    let b = 0.25;

    RGBColor {
        r: (r * 255.99) as u8,
        g: (g * 255.999) as u8,
        b: (b * 255.999) as u8,
    }
}

fn get_color(x: usize, y: usize, ray: Ray) -> RGBColor {
    if hit(ray) {
        RGBColor { r: 255, g: 0, b: 0 }
    } else {
        get_background_color(x, y)
    }
}

fn render_surface(
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
    cam: Camera,
    progress_bar: ProgressBar,
) -> Surface {
    let mut surface = Surface::new(x_offset, y_offset, width, height);
    for j in 0..height {
        progress_bar.inc(1);
        for i in 0..width {
            let u = (i + x_offset) as f64 / (IMG_WIDTH - 1) as f64;
            let v = (j + y_offset) as f64 / (IMG_HEIGHT - 1) as f64;
            let ray = cam.get_ray(u, v);
            let color = get_color(i + x_offset, j + y_offset, ray);
            surface.set_color(i, j, color);
        }
    }
    progress_bar.finish_with_message("Done");

    surface
}

fn main() {
    let camera = Camera::new(
        EYE,
        LOOKAT,
        UP,
        ASPECT_RATIO as f64,
        VFOV,
        APERTURE,
        DIST_TO_FOCUS,
    );

    // If I use more than 12 threads, the program crashed on my PC
    let available_threads = num_cpus::get();
    let thread_count = if available_threads > 12 {
        12
    } else {
        available_threads
    };

    let section_height = IMG_HEIGHT / thread_count;
    let mut extra_pixels = IMG_HEIGHT % thread_count;
    let (tx, rx) = channel();

    println!("Using {} threads", thread_count);

    // Progress bar setup
    let multi_progress = MultiProgress::new();
    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {prefix:>10}: {bar:40.cyan/blue} {pos:>5}/{len:5} {msg}")
        .progress_chars("=>-");

    let mut height_offset = 0;
    for i in 0..thread_count {
        let local_camera = camera.clone();

        let child_tx = tx.clone();

        // Spread extra pixels evenly across threads
        let surface_height = if extra_pixels > 0 {
            extra_pixels -= 1;
            section_height + 1
        } else {
            section_height
        };

        let progress_bar = multi_progress.add(ProgressBar::new(surface_height as u64));
        progress_bar.set_style(progress_style.clone());
        progress_bar.set_message("Scanlines remaining");
        let prefix = format!("Thread {}", i);
        progress_bar.set_prefix(prefix.as_str());

        thread::spawn(move || {
            child_tx
                .send(render_surface(
                    0,
                    height_offset,
                    IMG_WIDTH,
                    surface_height,
                    local_camera,
                    progress_bar,
                ))
                .unwrap();
        });

        height_offset += surface_height;
    }
    multi_progress.join().unwrap();

    drop(tx);

    let mut img = Surface::new(0, 0, IMG_WIDTH, IMG_HEIGHT);
    for result in rx.iter() {
        img.merge(&result);
    }

    img.save("image.png").unwrap();
}
