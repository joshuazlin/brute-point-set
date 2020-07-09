
use std::env;
use rug::{Rational, Integer};

mod geometry;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let n : usize = args[1].parse().unwrap();

    let mut g = _{
        vertices = vec![
        Point{
            x : Rational::from(0),
            y : Rational::from(0),
        },
        Point{
            x : Rational::from(1),
            y : Rational::from(0),
        },
        Point{
            x : Rational::from(0),
            y : Rational::from(1),
        }]
    } //Initialize a triangle.

    for _i in 0..n{
        let candidates = g.triangles.map(|t| g.clone().add_vertex(t.find_generic_point())); //add_vertex needs to return a copy of self
        let m = candidates.map(|c| num_simp_ham(c)).iter().enumerate().max_by_key(|(i,x)| x);
        println!("{}",m[1]);
        let g = m[0];
    }


}
