#[macro_use] extern crate rustonum;
extern crate ppmiors;

fn main()
{
    let res = ppmiors::read_ppm_p5("/home/daiver/pstorage/AlexF.pgm");
    println!("saving....");
    ppmiors::save_ppm_p5(&res, "tmp.pgm");
}
