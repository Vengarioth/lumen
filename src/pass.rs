
pub struct Pass {
    pub rect: Rect2d,
    pub buffer: Buffer,
    pub total_width: u32,
    pub total_height: u32,
    pub subsamples: u32,
}

impl Pass {
    pub fn new(rect: Rect2d, buffer: Buffer, total_width: u32, total_height: u32, subsamples: u32) -> Pass {
        Pass {
            rect,
            buffer,
            total_width,
            total_height,
            subsamples,
        }
    }
}


fn get_passes(width: u32, height: u32, slice_width: u32, slice_height: u32, subsamples: u32, progress_bar: ProgressBar) -> Vec<Pass> {
    let remainder_x = width % slice_width;
    let remainder_y = height % slice_height;

    let sx = (width - remainder_x) / slice_width;
    let sy = (height - remainder_y) / slice_height;

    let mut passes = Vec::new();

    for x in 0..sx {
        for y in 0..sy {
            let rect = Rect2d::new(x * slice_width, y * slice_height, slice_width, slice_height);
            let buffer = Buffer::new(slice_width, slice_height);
            passes.push(Pass::new(rect, buffer, width, height, subsamples, progress_bar.clone()));
        }
    }

    passes
}

fn render_pass(pass: &mut Pass, scene: &Scene, camera: &Camera) {
    let width = pass.total_width;
    let height = pass.total_height;
    let subsamples = pass.subsamples;
    let mut buffer = Buffer::new(pass.rect.width, pass.rect.height);
    for x in 0..pass.rect.width {
        for y in 0..pass.rect.height {
            let mut color = float3::new(0.0, 0.0, 0.0);
            for _ in 0..subsamples {
                let u = ((pass.rect.x + x) as f32 + rand::random::<f32>()) / width as f32;
                let v = ((pass.rect.y + y) as f32 + rand::random::<f32>()) / height as f32;

                let ray = camera.get_ray(u, v);
                // color += normal(&scene, &ray);
                color += trace(&scene, &ray, 0);
            }

            color /= subsamples as f32;

            let (r, g, b, _) = buffer.get_pixel(x, y);
            let old_color = float3::new(r as f32 / 255.9, g as f32 / 255.9, b as f32 / 255.9);
            color += old_color;
            color /= 2.0;

            let r = (color.x * 255.9) as u8;
            let g = (color.y * 255.9) as u8;
            let b = (color.z * 255.9) as u8;

            buffer.set_pixel(x, y, r, g, b, 255);
        }
    }
}
