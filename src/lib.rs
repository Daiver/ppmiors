
#[macro_use] extern crate rustonum;

use std::fs::File;
use std::io::prelude::*;

use rustonum::MatrixXf;

pub fn save_ppm_p2(mat : &MatrixXf, fname: &str)
{
    let mut f = File::create(fname).unwrap();
    f.write_all(b"P2\n").unwrap();
    f.write_all(format!("{} {}\n255\n", mat.cols(), mat.rows()).as_bytes()).unwrap();
    let values = mat.values().iter();
    for line_num in (0..mat.rows()){
        let line = values.clone().skip(line_num * mat.cols()).take(mat.cols())
            .map(|&x| (x as u8).to_string()).collect::<Vec<_>>().join(" ");
        f.write_all(line.as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
    }
}


pub fn read_ppm_p5(fname: &str) -> MatrixXf 
{
    let mut f = File::open(fname).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    f.read_to_end(&mut contents).unwrap();

    let contents_str = contents.clone().
                        iter().map(|&x| x as char).collect::<String>();
    let mut splt = contents_str.split("\n").filter(|&x| x.len() > 0);
    assert!(splt.next().unwrap() == "P5");
    let mut line = splt.next().unwrap();
    while (line).to_string().as_bytes()[0] == b'#' {
        line = splt.next().unwrap();
    }
    let shape = line.split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let rows = shape[1];
    let cols = shape[0];

    //pass comments
    line = splt.next().unwrap();
    while (line).to_string().as_bytes()[0] == b'#' {
        line = splt.next().unwrap();
    }
    
    assert!(line.parse::<usize>().unwrap() == 255);

    let mut res = MatrixXf::zeros(rows, cols);

    let line = splt.next().unwrap().chars();
    let mut row = 0;
    let mut col = 0;
    //println!("len {}", line.len());
    for item in line {
        res[(row, col)] = (item as u8) as f32;
        col += 1;
        if col >= cols {
            col  = 0;
            row += 1;
        }
    }
    assert!(row == rows);

    res
}
