
extern crate image;

use image::{GenericImage, GenericImageView};

fn main() {
    let mut img = image::open("/home/jbyer/shared/images/image1.png").unwrap();
    let corner_pixel = img.get_pixel(0, 0);
    let (width, height) = img.dimensions();
    
    //let mut out: RgbaImage = ImageBuffer::new(width, height);
    
    for w in 0..width {
        for h in 0..height {
            //let img_pixel = img.get_pixel(w, h);
            //let out_pixel = out.get_pixel(w, h);
            if img.get_pixel(w, h) == corner_pixel {
                img.put_pixel(w, h, image::Rgba([0,0,0,0]));
            }
        }
    }
    
    img.save("./images/image1_processed.png").unwrap();

    /*
    for pixel in out.enumerate_pixels_mut() {
        
        if pixel == corner_pixel {
            *pixel.2 = image::Rgba([0,0,0,0]);
        }
        else {
            *pixel.2 = pixel;
        }
        
    }
    */
    
    //println!("{:?}", corner_pixel);

     //out.save("./images/image1_processed.png").unwrap();
}

/*
fn blur_section() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("/home/jbyer/shared/images/image_compressor.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    let mut cropimg = imageops::crop(&mut img, 0, 0, 500, 500);
    let blurimg = imageops::blur(&cropimg, 20.0);
    cropimg.copy_from(&blurimg, 0, 0);

    // Write the contents of this image to the Writer in PNG format.
    img.save("./images/image_compressor.png").unwrap();
}
*/