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
use ray_math::{Camera, Color, Hittable, Ray};
use rayon::prelude::*;
use scenes::SceneOption;

mod scenes;

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

    let mut rand = rand::thread_rng();
    let world = scenes::make_scene(&mut rand, SceneOption::TwoSpheres);

    // Image
    let image_width = 400;
    let image_height = ((image_width as f64) / world.camera.aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let motion_time_range = 0.0..1.0;

    let camera = Camera::new(world.camera);

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

    let root = world.root;
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
                let ray =
                    camera.get_ray_defocused(&mut rand, Some(motion_time_range.clone()), u, v);
                pixel_color += ray_color(&ray, &mut rand, &root, max_depth);
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

    let mut f = File::create(file)?;
    write!(f, "P3\n{} {}\n255\n", image_width, image_height)?;
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
