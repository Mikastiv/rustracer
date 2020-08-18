mod axis_aligned_bb;
mod bvh_node;
mod hittable;
mod material;
mod math;
mod program_args;
mod ray;
mod render_options;
mod rgbcolor;
mod scene;
mod surface;
mod vec3;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::prelude::*;
use std::{
    env,
    fs::File,
    io, process,
    sync::{mpsc::channel, Arc},
    thread,
};

use hittable::{Hittable, HittableList};
use material::Material;
use program_args::ProgramArgs;
use ray::Ray;
use render_options::RenderOptions;
use rgbcolor::RGBColor;
use scene::{Camera, Config, Scene};
use surface::Surface;
use vec3::{Color, Vec3};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut rng = thread_rng();

    let ground_material = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };

    let ground = Hittable::Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    };
    world.add(Arc::new(ground));

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
                    let center1 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);

                    let sphere = Hittable::MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Material::Lambertian { albedo },
                    };

                    world.add(Arc::new(sphere));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_color_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);

                    let sphere = Hittable::Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal { albedo, fuzz },
                    };

                    world.add(Arc::new(sphere));
                } else {
                    let sphere = Hittable::Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric { ref_idx: 1.5 },
                    };

                    world.add(Arc::new(sphere));
                }
            }
        }
    }

    let glass_sphere = Hittable::Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric { ref_idx: 1.5 },
    };
    world.add(Arc::new(glass_sphere));

    let mat_sphere = Hittable::Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    };
    world.add(Arc::new(mat_sphere));

    let metal_sphere = Hittable::Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    };
    world.add(Arc::new(metal_sphere));

    world
}

fn scale_color(color: Color, spp: u32) -> RGBColor {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / spp as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    RGBColor::new(
        (256.0 * math::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * math::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * math::clamp(b, 0.0, 0.999)) as u8,
    )
}

fn ray_color(
    ray: Ray,
    world_ptr: Arc<HittableList>,
    options: &RenderOptions,
    y: usize,
    depth: u32,
) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(intersection) = world_ptr.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some((attenuation, scattered)) = intersection.material.scatter(ray, &intersection) {
            attenuation * ray_color(scattered, world_ptr, &options, y, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        options
            .background
            .get_color(y as f64 / options.img_height as f64)
    }
}

fn render_surface(
    x_offset: usize,
    y_offset: usize,
    height: usize,
    options: RenderOptions,
    cam: Camera,
    world_ptr: Arc<HittableList>,
    progress_bar: ProgressBar,
) -> Surface {
    let mut rng = thread_rng();
    let mut surface = Surface::new(x_offset, y_offset, options.img_width, height);

    let mut msg_str_len = 0;
    for j in 0..height {
        let msg = format!("Rendering scanline #{}", j + 1);
        msg_str_len = msg.len();
        progress_bar.set_message(msg.as_str());
        progress_bar.inc(1);

        for i in 0..options.img_width {
            let mut color = Color::new(0.0, 0.0, 0.0);

            if i % options.progress_tick_rate == 0 {
                progress_bar.tick();
            }

            for _s in 0..options.sample_per_pixel {
                let u = ((i + x_offset) as f64 + rng.gen::<f64>()) / (options.img_width - 1) as f64;
                let v =
                    ((j + y_offset) as f64 + rng.gen::<f64>()) / (options.img_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                color += ray_color(
                    ray,
                    world_ptr.clone(),
                    &options,
                    j + y_offset,
                    options.max_depth,
                );
            }

            surface.set_color(i, j, scale_color(color, options.sample_per_pixel));
        }
    }

    // 5 is length of str "Done "
    progress_bar.finish_with_message(format!("Done {:len$}", " ", len = msg_str_len - 5).as_str());

    surface
}

fn parse_file(file_path: &str) -> Result<Config, io::Error> {
    let file = File::open(file_path)?;

    let data = serde_json::from_reader(file)?;
    Ok(data)
}

fn parse_args() -> Result<ProgramArgs, String> {
    let args: Vec<String> = env::args().collect();
    if !(args.len() == 2 || args.len() == 3) {
        Err(format!(
            "Usage: {} config_file.json job_count (note: job_count is optional)",
            &args[0]
        ))
    } else if args.len() == 2 {
        let file_path = args[1].clone();
        Ok(ProgramArgs {
            file_path,
            job_count: 0,
        })
    } else {
        let file_path = args[1].clone();
        match args[2].parse() {
            Ok(job_count) => Ok(ProgramArgs {
                file_path,
                job_count,
            }),
            Err(_) => Err("Job count must be an unsigned number".to_string()),
        }
    }
}

fn get_job_count(arg: usize) -> usize {
    let available_threads = num_cpus::get();
    if arg == 0 || arg > available_threads {
        available_threads - 1
    } else {
        arg
    }
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let cfg: Config = match parse_file(args.file_path.as_str()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let scene = Scene::new(&cfg, random_scene());

    let thread_count = get_job_count(args.job_count);
    println!("Using {} threads", thread_count);

    // Each thread renders an image wide strip of the final image like shown below
    // --------------------------------------
    // |                                    |
    // --------------------------------------
    // |                                    |
    // --------------------------------------
    // |                                    |
    // --------------------------------------
    let section_height = cfg.img_height / thread_count;
    let mut extra_pixels = cfg.img_height % thread_count;

    let (tx, rx) = channel();

    // Multi progress bars setup
    let multi_progress = MultiProgress::new();
    multi_progress.set_move_cursor(true);
    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {prefix:>10}: {bar:40.yellow/cyan} {pos:>5}/{len:5} {msg}")
        .progress_chars("=>-");

    let render_options = RenderOptions::new(
        cfg.progress_tick_rate,
        cfg.img_width,
        cfg.img_height,
        cfg.sample_per_pixel,
        cfg.max_depth,
        cfg.background,
    );

    let mut height_offset = 0;
    for i in 0..thread_count {
        let camera = scene.get_camera();
        let world_ptr = scene.get_objects();
        let local_options = render_options.clone();

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
            if let Err(err) = child_tx.send(render_surface(
                0,
                height_offset,
                surface_height,
                local_options,
                camera,
                world_ptr,
                progress_bar,
            )) {
                eprintln!("{}", err);
            }
        });

        height_offset += surface_height;
    }

    if let Err(err) = multi_progress.join() {
        eprintln!("{}", err);
    };

    drop(tx);

    // Merge every portion of the image in output image
    let mut img = Surface::new(0, 0, render_options.img_width, render_options.img_height);
    for result in rx.iter() {
        img.merge(&result);
    }

    if let Err(err) = img.save("output.png") {
        eprintln!("{}", err);
    }
}
