use std::fs::File;
use std::io::prelude::*;

use rand::Rng;
use ray_math::{Camera, Color, Hittable, HittableList, Point3, Ray, Sphere};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (hit.normal() + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t)
}

fn write_image(file: &str) -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let camera = Camera::new(aspect_ratio);

    // Render
    let mut f = File::create(file)?;
    write!(f, "P3\n{} {}\n255\n", image_width, image_height)?;

    let world = HittableList::from(vec![
        Box::new(Sphere::from(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::from(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    let mut rand = rand::thread_rng();

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {:<6}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand.gen_range(0.0..=1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rand.gen_range(0.0..=1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            (pixel_color / samples_per_pixel as f64).write_color(&mut f)?;
        }
    }

    println!("\nDone.");

    Ok(())
}

fn main() {
    if let Err(_) = write_image("image.ppm") {
        eprintln!("Failed to generate image");
    }
}
