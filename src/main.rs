mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use material::Dielectric;
use material::Lambertian;
use material::Material;
use material::Metal;
use rand::thread_rng;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;

fn output_color(color: &Color, samples_per_pixel: i32) -> String {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;
    format!("{} {} {}", ir, ig, ib)
}

fn ray_color(initial_ray: &Ray, world: &dyn Hittable, max_depth: i32) -> Color {
    let mut current_ray = initial_ray.clone();
    let mut attenuation = Color::new(1.0, 1.0, 1.0);
    for _ in 0..max_depth {
        if let Some(hit) = world.hit(&current_ray, 0.001, f64::INFINITY) {
            if let Some((multiplicitive_attenuation, scattered)) =
                hit.material.scatter(&current_ray, &hit)
            {
                attenuation *= multiplicitive_attenuation;
                current_ray = scattered;
                continue;
            }
            break;
        }

        let unit_direction = current_ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return attenuation
            * ((1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0));
    }

    Color::zero()
}

// fn random_scene() -> HittableList {

//     world
// }

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let mut materials = vec![];
    for _ in -11..11 {
        let mut row_materials: Vec<Box<dyn Material>> = vec![];
        for _ in -11..11 {
            let choose_material = rand::random::<f64>();
            if choose_material < 0.8 {
                let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                row_materials.push(Box::new(Lambertian::new(albedo)));
            } else if choose_material < 0.95 {
                let albedo = Color::random(0.5, 1.0);
                let fuzz = thread_rng().gen_range(0.0..0.5);
                row_materials.push(Box::new(Metal::new(albedo, fuzz)));
            } else {
                row_materials.push(Box::new(Dielectric::new(1.5)));
            }
        }
        materials.push(row_materials);
    }

    let mut world = HittableList::new();
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            world.add(Sphere::new(
                center,
                0.2,
                &*materials[(a + 11) as usize][(b + 11) as usize],
            ));
        }
    }

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &material1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &material2));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &material3));

    // Camera
    let lookfrom = Point3::new(12.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let focus_distance = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_distance,
    );

    // Render
    let mut output = String::new();
    output += "P3\n";
    output += &format!("{} {}\n", image_width, image_height);
    output += "255\n";

    for j in (0..image_height).rev() {
        eprint!("\r\x1b[KScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / ((image_width - 1) as f64);
                let v = (j as f64 + rand::random::<f64>()) / ((image_height - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            output += &format!("{}\n", output_color(&pixel_color, samples_per_pixel));
        }
    }
    eprintln!("\nDone.");

    print!("{}", output);
}
