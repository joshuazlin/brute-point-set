
use std::env;
use rug::{Rational, Integer};

mod geometry;
use geometry::*;

struct PartialCycleGraph{
    vg : VisibilityGraph,
    cycle : Vec<usize>,
}

impl PartialCycleGraph{
    fn vertices(&self) -> Vec<Point>{
        self.vg.vertices()
    }

    fn edges(&self) -> Vec<[usize;2]>{
        self.vg.edges()
    }

}

fn num_simp_ham(pcg : PartialCycleGraph) -> usize {
    //Calculates the number of crossing-free hamiltonian cycles
    //for a given list of points in general position. 
    //This counts all the cycles twice, but who cares; I do!

    if pcg.cycle.len() == pcg.vertices().len(){
        return 1;
    }

    let mut result = 0;
    for i in pcg.visible(*pcg.cycle.last().unwrap()){ //Loop over vertices that I can see from the endpoint
        if pcg.cycle.contains(&i){continue}; //cycle stored as a list of indices
        let mut temp_g = pcg.clone(); //This is expensive and bad, but too bad!
        temp_g.add_edge([*pcg.cycle.last().unwrap(),i]);
        result += num_simp_ham(temp_g);
    }
    result
}


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
