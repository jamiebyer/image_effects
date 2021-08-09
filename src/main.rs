/*
Fluid tutorial: https://www.youtube.com/watch?v=k_P0wG3-dNk
*/
extern crate image;
extern crate fltk;

use image::{GenericImage, GenericImageView};
use fltk::{prelude::*, app::*, button::*, frame::*, window::*, *};

use std::path::Path;


fn main() {
    
    let app = App::default();
    let mut wind = Window::new(100, 100, 600, 500, "Image Effects");
    let mut but = Button::new(200, 260, 80, 40, "Click me!");
    let _input_image_label = Frame::new(100, 100, 200, 30, "Input Image");
    let mut input_image = menu::Choice::new(100, 150, 200, 30, None);
    input_image.add_choice("Image 1|Image 2|Image 3|Image 4|Image 5");
    input_image.set_value(0);

    wind.end();
    wind.show();

    // Remember: Callbacks after initializing the interface
    but.set_callback(move |_| {
        clear_solid_background(input_image.value());
        //println!("Image Choice: {:?}", input_image.value());
    });

    app.run().unwrap();
}


fn clear_solid_background(image_number: i32) {
    let mut path_ext = "";
    if image_number == 0 {
        path_ext = "image1.png";
    } else if image_number == 1 {
        path_ext = "image2.jpg";
    } else if image_number == 2 {
        path_ext = "image3.jpg";
    } else if image_number == 3 {
        path_ext = "image4.jpg";
    } else if image_number == 4 {
        path_ext = "image5.jpg";
    }

    let path = Path::new(".");
    let inp_path = path.join("/home/jamie/Desktop/RustProjects/image_effects/input_images/").join(path_ext);
    let mut img = image::open(inp_path).unwrap();
    let corner_pixel = img.get_pixel(0, 0);
    let (width, height) = img.dimensions();
    
    for w in 0..width {
        for h in 0..height {
            if img.get_pixel(w, h) == corner_pixel {
                img.put_pixel(w, h, image::Rgba([0,0,0,0]));
            }
        }
    }
    
    let out_path = path.join("/home/jamie/Desktop/RustProjects/image_effects/output_images/").join(path_ext);
    img.save(out_path).unwrap();
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