const IMAGE_HEIGHT: u32 = 256;
const IMAGE_WIDTH: u32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_HEIGHT, IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH as f64 - 1f64);
            let g = j as f64 / (IMAGE_HEIGHT as f64 - 1f64);
            const B: f64 = 0.25f64;
            let ir = (255.299 * r) as usize;
            let ig = (255.299 * g) as usize;
            let ib = (255.299 * B) as usize;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
