#[macro_use] extern crate rustonum;
extern crate ppmiors;

use rustonum::MatrixXf;

fn main()
{
    let mat_r = mat![255.0, 255.0; 0.0, 0.0];
    let mat_g = mat![0.0, 0.0; 255.0, 0.0];
    let mat_b = mat![0.0, 0.0; 0.0, 255.0];
    ppmiors::save_ppm_p3(&mat_r, &mat_g, &mat_b, "tmp.ppm");
}
