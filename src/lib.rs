
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

pub fn save_ppm_p3(
    mat_r : &MatrixXf, 
    mat_g : &MatrixXf, 
    mat_b : &MatrixXf, 
    fname: &str)
{
    assert!(mat_r.cols() == mat_b.cols() && mat_r.cols() == mat_g.cols());
    assert!(mat_r.rows() == mat_b.rows() && mat_r.rows() == mat_g.rows());
    let mut f = File::create(fname).unwrap();
    f.write_all(b"P3\n").unwrap();
    f.write_all(format!("{} {}\n255\n", mat_r.cols(), mat_r.rows()).as_bytes()).unwrap();
    let values = mat_r.values().iter().zip(mat_g.values().iter()).zip(mat_b.values().iter());
    for line_num in (0..mat_r.rows()){
        let line = values.clone().skip(line_num * mat_r.cols()).take(mat_r.cols())
            .map(| ((&r, &g), &b) | format!("{} {} {}", r as u8, g as u8, b as u8))
            .collect::<Vec<_>>().join(" ");
        f.write_all(line.as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
    }
}

pub fn save_ppm_p5(
    mat : &MatrixXf, 
    fname: &str)
{
    let mut f = File::create(fname).unwrap();
    f.write_all(b"P5\n").unwrap();
    f.write_all(format!("{} {}\n255\n", mat.cols(), mat.rows()).as_bytes()).unwrap();
    let data_to_write: Vec<u8> = mat.values().iter().map(|&x| x as u8).collect::<Vec<_>>();
    f.write_all(&data_to_write).unwrap();
}

pub fn save_ppm_p6(
    mat_r : &MatrixXf, 
    mat_g : &MatrixXf, 
    mat_b : &MatrixXf, 
    fname: &str)
{
    assert!(mat_r.cols() == mat_b.cols() && mat_r.cols() == mat_g.cols());
    assert!(mat_r.rows() == mat_b.rows() && mat_r.rows() == mat_g.rows());
    let mut f = File::create(fname).unwrap();
    f.write_all(b"P6\n").unwrap();
    f.write_all(format!("{} {}\n255\n", mat_r.cols(), mat_r.rows()).as_bytes()).unwrap();
    let zipped_values = mat_r.values().iter().zip(mat_g.values().iter()).zip(mat_b.values().iter());
    let mut data_to_write: Vec<u8> = Vec::new();
    for ((&r, &g), &b) in zipped_values {
        data_to_write.push(r as u8);
        data_to_write.push(g as u8);
        data_to_write.push(b as u8);
    }
    f.write_all(&data_to_write).unwrap();
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


pub fn read_ppm_p6(fname: &str) -> [MatrixXf; 3]
{
    let mut f = File::open(fname).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    f.read_to_end(&mut contents).unwrap();

    let contents_str = contents.clone().
                        iter().map(|&x| x as char).collect::<String>();
    let mut splt = contents_str.split("\n").filter(|&x| x.len() > 0);
    assert!(splt.next().unwrap() == "P6");
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

    let mut res_t = [MatrixXf::zeros(rows, cols),
                     MatrixXf::zeros(rows, cols),
                     MatrixXf::zeros(rows, cols)];

    let line = splt.next().unwrap().chars();
    let mut row       = 0;
    let mut col       = 0;
    let mut component = 0;

    for item in line {
        res_t[component][(row, col)] = (item as u8) as f32;
        component += 1;
        if component > 2 {
            component = 0;
            col += 1;
            if col >= cols {
                col  = 0;
                row += 1;
            }
        }
    }
    assert!(row == rows);

    res_t
}
