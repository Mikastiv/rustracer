mod camera;
mod hittable;
mod hittable_list;
mod material;
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
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use rgbcolor::RGBColor;
use sphere::Sphere;
use surface::Surface;
use vec3::{Color, Vec3};

// Update tick rate of each progress bars (in pixel)
// Every X pixel rendered, tick once
const PROGRESS_BARS_TICK_RATE: usize = 30;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: usize = 1200;
const IMG_HEIGHT: usize = (IMG_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLE_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 40;

const VFOV: f64 = 20.0;
const EYE: Vec3 = Vec3::new(13.0, 2.0, 3.0);
const LOOKAT: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const DIST_TO_FOCUS: f64 = 10.0;
const APERTURE: f64 = 0.1;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut rng = thread_rng();

    let ground_material = Box::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_color() * Vec3::random_color();
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian { albedo }),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_color_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal { albedo, fuzz }),
                    )));
                } else {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric { ref_idx: 1.5 }),
                    )));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric { ref_idx: 1.5 }),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    )));

    world
}

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
        if let Some((attenuation, scattered)) = intersection.material.scatter(ray, &intersection) {
            attenuation * ray_color(scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
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

    let mut msg_str_len = 0;
    for j in 0..height {
        let msg = format!("Rendering scanline #{}", j + 1);
        msg_str_len = msg.len();
        progress_bar.set_message(msg.as_str());
        progress_bar.inc(1);

        for i in 0..width {
            let mut color = Color::new(0.0, 0.0, 0.0);

            if i % PROGRESS_BARS_TICK_RATE == 0 {
                progress_bar.tick();
            }

            for _s in 0..SAMPLE_PER_PIXEL {
                let u = ((i + x_offset) as f64 + rng.gen::<f64>()) / (IMG_WIDTH - 1) as f64;
                let v = ((j + y_offset) as f64 + rng.gen::<f64>()) / (IMG_HEIGHT - 1) as f64;
                let ray = cam.get_ray(u, v);
                color += ray_color(ray, world.deref(), MAX_DEPTH);
            }

            surface.set_color(i, j, scale_color(color));
        }
    }
    // 4 is length of str "Done"
    progress_bar.finish_with_message(format!("Done {:len$}", " ", len = msg_str_len - 4).as_str());

    surface
}

fn main() {
    let world = Arc::new(RwLock::new(random_scene()));
    let camera = Camera::new(EYE, LOOKAT, UP, ASPECT_RATIO, VFOV, APERTURE, DIST_TO_FOCUS);

    // Note: on one of my PC, I have to lower the thread count manually from 16 to 12
    // ohterwise the console doesn't redraw each progress line over it's position, but
    // spams the console and draw new lines at every tick
    // let thread_count = num_cpus::get();
    let thread_count = 8;
    println!("Using {} threads", thread_count);

    // Each thread renders an image wide strip of the final image like shown below
    // --------------------------------------
    // |                                    |
    // --------------------------------------
    // |                                    |
    // --------------------------------------
    // |                                    |
    // --------------------------------------
    let section_height = IMG_HEIGHT / thread_count;
    let mut extra_pixels = IMG_HEIGHT % thread_count;
    let (tx, rx) = channel();

    // Multi progress bars setup
    let multi_progress = MultiProgress::new();
    multi_progress.set_move_cursor(true);
    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {prefix:>10}: {bar:40.yellow/cyan} {pos:>5}/{len:5} {msg}")
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

        // Individual progress bar setup
        let progress_bar = multi_progress.add(ProgressBar::new(surface_height as u64));
        progress_bar.set_style(progress_style.clone());
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

    // Merge every portion of the image in output image
    let mut img = Surface::new(0, 0, IMG_WIDTH, IMG_HEIGHT);
    for result in rx.iter() {
        img.merge(&result);
    }

    img.save("output.png").unwrap();
}
