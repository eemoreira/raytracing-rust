use crate::hit::Hittable;
use crate::rand;
use crate::{Interval, Ray, Rgb, Vec3, World};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    viewport_v: Vec3,
    viewport_u: Vec3,
}

const FOCAL_LENGTH: f64 = 1.0;
const RAY_TRACING_DEPTH: i32 = 50;
const SAMPLES_PER_PIXEL: u8 = 25;
const ANTI_ALIASING_FACTOR: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;

impl Camera {
    pub fn new(aspect_ratio: f64, image_height: usize, viewport_height: f64) -> Camera {
        let image_width = (image_height as f64 * aspect_ratio) as usize;
        let image_width = if image_width == 0 { 1 } else { image_width };
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        Camera {
            image_width,
            image_height,
            viewport_v: Vec3::new(0.0, -viewport_height, 0.0),
            viewport_u: Vec3::new(viewport_width, 0.0, 0.0),
        }
    }

    fn ray_color(world: &World, ray: Ray, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new_eq(0.0);
        }

        if let Some(hit) = world.hit(&ray, Interval::new(0.0001, f64::INFINITY)) {
            let mat = hit.material();
            if let Some((attenuation, scattered)) = mat.scatter(&ray, &hit) {
                Self::ray_color(world, scattered, depth - 1) * attenuation
            } else {
                mat.emit()
            }
        } else {
            Vec3::new(0.02, 0.02, 0.1)
        }
    }

    pub fn render(
        &self,
        num_threads: usize,
        world: World,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create("image.ppm")?;
        let mut out = BufWriter::new(file);
        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        let block_size = (self.image_width * self.image_height + num_threads - 1) / num_threads;
        println!("block size = {}", block_size);

        let pixels: Vec<Vec<(usize, usize, Rgb)>> = std::thread::scope(|s| {
            let handles: Vec<_> = (0..num_threads)
                .map(|id| {
                    let world = &world;
                    s.spawn(move || {
                        let l = id * block_size;
                        let r = (l + block_size - 1).min(self.image_width * self.image_height - 1);
                        println!("(id, l, r) = ({}, {}, {})", id, l, r);
                        let thread_ans: Vec<(usize, usize, Rgb)> = (l..=r)
                            .map(|x| {
                                let i = x / self.image_width;
                                let j = x % self.image_width;
                                let color: Vec3 = (0..SAMPLES_PER_PIXEL)
                                    .map(move |_| {
                                        let offset = rand::random_square();
                                        let pixel_center = self.upper_left_pixel()
                                            + self.pixel_step_v() * (i as f64 + offset.x())
                                            + self.pixel_step_u() * (j as f64 + offset.y());

                                        let ray_direction =
                                            (pixel_center - self.center()).unit_vector();
                                        let ray = Ray::new(self.center(), ray_direction);
                                        Camera::ray_color(world, ray, RAY_TRACING_DEPTH)
                                    })
                                    .sum();
                                (i, j, Rgb::to_rgb(color * ANTI_ALIASING_FACTOR))
                            })
                            .collect();
                        thread_ans
                    })
                })
                .collect();
            let handle: Result<Vec<Vec<(usize, usize, Rgb)>>, _> =
                handles.into_iter().map(|h| h.join()).collect();
            handle
        })
        .map_err(|_| "render thread panic")?;

        let mut image: Vec<Rgb> =
            vec![Rgb::to_rgb(Vec3::new_eq(0.0)); self.image_width * self.image_height];

        for (i, j, color) in pixels.into_iter().flatten() {
            image[i * self.image_width + j] = color;
        }
        for i in 0..self.image_height {
            for j in 0..self.image_width {
                image[i * self.image_width + j].write(&mut out)?;
            }
        }
        Ok(())
    }

    pub fn center(&self) -> Vec3 {
        Vec3::new_eq(0.0)
    }

    pub fn pixel_step_u(&self) -> Vec3 {
        self.viewport_u / self.image_width as f64
    }
    pub fn pixel_step_v(&self) -> Vec3 {
        self.viewport_v / self.image_height as f64
    }

    pub fn upper_left_pixel(&self) -> Vec3 {
        let viewport_upper_left = self.center()
            - self.viewport_u / 2.0
            - self.viewport_v / 2.0
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        viewport_upper_left + (self.pixel_step_u() + self.pixel_step_v()) * 0.5
    }
}
