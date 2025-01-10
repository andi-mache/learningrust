fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {} \n255", image_width, image_height);

    for mut j in 0..image_height{
        j += 1;
        for mut i in 0..image_width {
            i += 1;

            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            //println!("i is {}" ,i);

            let ir = 255.999 * r;
            let ig = 255.999 * g;
            let ib = 255.999 * b;

            //println!(" r is {} g is {} b is {}",r,g,b);

            println!("{} {} {}", ir as u32, ig as u32, ib as u32);
        }
    }
}
