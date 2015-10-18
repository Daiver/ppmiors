#[macro_use] extern crate rustonum;
extern crate ppmiors;

fn main()
{
    let res = ppmiors::read_ppm_p6("/home/daiver/pstorage/AlexF.ppm");
    println!("saving....");
    ppmiors::write_ppm_p6(&res, "tmp.pgm");
}
