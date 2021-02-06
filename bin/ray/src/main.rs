use std::io::prelude::*;
use std::{fs::File, rc::Rc};

use rand::Rng;
use ray_math::{
    material::{Dielectric, Lambertian, Metal},
    Camera, Color, Hittable, HittableList, Point3, Ray, Sphere,
};

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
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio);

    // Render
    let mut f = File::create(file)?;
    write!(f, "P3\n{} {}\n255\n", image_width, image_height)?;

    let world = {
        let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let left = Rc::new(Dielectric::new(1.5));
        let right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
        HittableList::from(vec![
            Box::new(Sphere::from(Point3::new(0.0, -100.5, -1.0), 100.0, ground)),
            Box::new(Sphere::from(Point3::new(0.0, 0.0, -1.0), 0.5, center)),
            Box::new(Sphere::from(Point3::new(-1.0, 0.0, -1.0), 0.5, left)),
            Box::new(Sphere::from(Point3::new(1.0, 0.0, -1.0), 0.5, right)),
        ])
    };

    let mut rand = rand::thread_rng();

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {:<6}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand.gen_range(0.0..=1.0)) / (image_width - 1) as f64;
                let v = (j as f64 + rand.gen_range(0.0..=1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &mut rand, &world, max_depth);
            }

            let scale = 1.0 / samples_per_pixel as f64;
            let adjusted_color = Color::new(
                (scale * pixel_color.x()).sqrt(),
                (scale * pixel_color.y()).sqrt(),
                (scale * pixel_color.z()).sqrt(),
            );
            adjusted_color.write_color(&mut f)?;
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
