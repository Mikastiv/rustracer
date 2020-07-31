mod camera;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod rgbcolor;
mod sphere;
mod surface;
mod vec3;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::prelude::*;
use std::{
    ops::Deref,
    sync::{mpsc::channel, Arc, RwLock},
    thread,
};

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use rgbcolor::RGBColor;
use sphere::Sphere;
use surface::Surface;
use vec3::{Color, Vec3};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: usize = 400;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLE_PER_PIXEL: u32 = 50;
const MAX_DEPTH: u32 = 50;

const VFOV: f64 = 20.0;
const EYE: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 3.0,
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

fn scale_color(color: Color) -> RGBColor {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / SAMPLE_PER_PIXEL as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    RGBColor::new(
        (256.0 * math::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * math::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * math::clamp(b, 0.0, 0.999)) as u8,
    )
}

fn ray_color(ray: Ray, world: &RwLock<dyn Hittable>, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let world_data = world.read().unwrap();

    if let Some(intersection) = world_data.hit(ray, 0.001, std::f64::INFINITY) {
        let target = intersection.point
            + intersection.normal
            + Vec3::random_in_hemisphere(intersection.normal);
        let bounced_ray = Ray::new(
            intersection.point,
            (target - intersection.point).normalize(),
        );
        0.5 * ray_color(bounced_ray, world, depth - 1)
    } else {
        let t = 0.5 * (ray.dir.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.4, 0.6, 1.0)
    }
}

fn render_surface(
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
    cam: Camera,
    world: Arc<RwLock<dyn Hittable>>,
    progress_bar: ProgressBar,
) -> Surface {
    let mut rng = thread_rng();
    let mut surface = Surface::new(x_offset, y_offset, width, height);

    for j in 0..height {
        progress_bar.inc(1);
        for i in 0..width {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..SAMPLE_PER_PIXEL {
                let u = ((i + x_offset) as f64 + rng.gen::<f64>()) / (IMG_WIDTH - 1) as f64;
                let v = ((j + y_offset) as f64 + rng.gen::<f64>()) / (IMG_HEIGHT - 1) as f64;
                let ray = cam.get_ray(u, v);
                color += ray_color(ray, world.deref(), MAX_DEPTH);
            }

            surface.set_color(i, j, scale_color(color));
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

    let world = Arc::new(RwLock::new(HittableList::new()));
    {
        let mut world_data = world.write().unwrap();
        let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
        let sphere2 = Sphere::new(Vec3::new(0.0, -100.0, -10.0), 100.0);
        world_data.add(Box::new(sphere1));
        world_data.add(Box::new(sphere2));
    }

    // If I use more than 12 threads, the program crashed on my PC
    let available_threads = num_cpus::get();
    let thread_count = if available_threads > 12 {
        10
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
        let local_world = world.clone();

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
        progress_bar.set_prefix(format!("Thread {}", i).as_str());

        thread::spawn(move || {
            child_tx
                .send(render_surface(
                    0,
                    height_offset,
                    IMG_WIDTH,
                    surface_height,
                    local_camera,
                    local_world,
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
