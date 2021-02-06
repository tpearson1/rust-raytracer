use std::{
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use rand::Rng;
use ray_math::{
    material::{Dielectric, Lambertian, Metal},
    Camera, CameraConfig, Color, Hittable, HittableList, Point3, Ray, Sphere, Vec3,
};
use rayon::prelude::*;

fn ray_color(ray: &Ray, rng: &mut dyn rand::RngCore, world: &dyn Hittable, depth: usize) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scatter_result) = hit.material().scatter(rng, &hit, ray) {
            let color = ray_color(&scatter_result.scattered, rng, world, depth - 1);
            return scatter_result.attenuation * color;
        }

        return Color::zero();
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::lerp(Color::one(), Color::new(0.5, 0.7, 1.0), t)
}

fn write_image(file: &str) -> std::io::Result<()> {
    println!("Starting");

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::zero();
    let camera = Camera::new(CameraConfig {
        look_from,
        look_at,
        view_up: Vec3::new(0.0, 1.0, 0.0),
        vertical_field_of_view_degrees: 20.0,
        aspect_ratio,
        aperture: 0.1,
        focus_distance: 10.0,
    });

    println!("Configured Camera");

    // Render
    let mut f = File::create(file)?;
    write!(f, "P3\n{} {}\n255\n", image_width, image_height)?;

    let mut rand = rand::thread_rng();
    let world = random_scene(&mut rand);

    println!("Configured Scene, starting to render");

    let pixels_done = Arc::new(AtomicUsize::new(0));
    let total = image_width * image_height;

    let join_handle = {
        let pixels_done_reader = Arc::clone(&pixels_done);
        std::thread::spawn(move || loop {
            let done = pixels_done_reader.load(Ordering::Relaxed);
            print!(
                "\rProgress: {}/{} pixels, {:.2}% done",
                done,
                total,
                100.0 * done as f64 / total as f64
            );

            std::io::stdout().flush().expect("Couldn't flush stdout");
            if done == total {
                println!("");
                break;
            }

            std::thread::sleep(Duration::from_millis(500));
        })
    };

    let mut pixels: Vec<_> = (0..image_height)
        .rev()
        .flat_map(|j| (0..image_width).map(move |i| (i, j)))
        .enumerate()
        .par_bridge()
        .map(|(idx, (i, j))| {
            let mut rand = rand::thread_rng();
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand.gen_range(0.0..=1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rand.gen_range(0.0..=1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray_defocused(&mut rand, u, v);
                pixel_color += ray_color(&ray, &mut rand, &world, max_depth);
            }

            let scale = 1.0 / samples_per_pixel as f64;
            let adjusted_color = Color::new(
                (scale * pixel_color.x()).sqrt(),
                (scale * pixel_color.y()).sqrt(),
                (scale * pixel_color.z()).sqrt(),
            );

            pixels_done.fetch_add(1, Ordering::Relaxed);
            (idx, adjusted_color)
        })
        .collect::<Vec<_>>();
    pixels.par_sort_unstable_by_key(|(idx, _)| *idx);

    join_handle.join().expect("Counter thread panicked!");
    println!("Rendered, saving");

    for (_, pixel) in pixels {
        pixel.write_color(&mut f)?;
    }

    println!("Done.");

    Ok(())
}

fn main() {
    if let Err(_) = write_image("image.ppm") {
        eprintln!("Failed to generate image");
    }
}

fn random_scene(rng: &mut dyn rand::RngCore) -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::from(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0..20);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..=1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let object = match choose_mat {
                19 => {
                    // Glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    Sphere::from(center, 0.2, mat)
                }
                16 | 17 | 18 => {
                    // Metal
                    let albedo = Color::random(rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    Sphere::from(center, 0.2, mat)
                }
                _ => {
                    // Diffuse
                    let albedo = Color::random_unit(rng) * Color::random_unit(rng);
                    let mat = Arc::new(Lambertian::new(albedo));
                    Sphere::from(center, 0.2, mat)
                }
            };

            world.add(Box::new(object));
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::from(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat1,
    )));

    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::from(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat2,
    )));

    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::from(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat3,
    )));

    world
}
