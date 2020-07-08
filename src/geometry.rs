/*
Hopefully implements arbitrary precision rational geometry in 2D.
*/

use rug::{Rational, Integer};

#[derive(Clone)]
struct Point{x: Rational, y : Rational}

impl Point{
    fn subtract(&self,p : &Point) -> Vect2D{
        //calculates self - p as a vector
        Vect2D{
            x : Rational::from(&self.x - &p.x),
            y : Rational::from(&self.y - &p.y),
        }
    }

    fn project_to(&self,e : &Edge) -> Rational{
        //projects down onto e, where e has scale 1?
        let v = e.as_vect(); 
        (self.subtract(&e.v1).dot(&v))/(v.norm())
    }
}

#[derive(Clone)]
struct Vect2D{x : Rational, y : Rational}
impl Vect2D{
    fn det(&self,v : &Vect2D) -> Rational{
        //calculates the magnitude of self x v; in other words
        //the determinant. 
        Rational::from(&self.x*&v.y) - Rational::from(&self.y*&v.x)
    }

    fn dot(&self, v : &Vect2D) -> Rational{
        Rational::from(&self.x*&v.x) + Rational::from(&self.y*&v.y)
    }

    fn norm(&self) -> Rational{
        self.dot(self)
    }
}

#[derive(Clone)]
struct Edge{v1 : Point, v2 : Point}

impl Edge{

    fn as_vect(&self) -> Vect2D{
        self.v2.subtract(&self.v1)
    }

    fn orient_point(&self, p : &Point) -> Option<bool>{
        //returns if self.v1, self.v2, p is anticlockwise;
        //in other words, if p is on the left of the edge
        let v2 = (*p).subtract(&self.v1);
        let det = self.as_vect().det(&v2);
        if det > 0{
            return Some(true);
        }
        else if det < 0{
            return Some(false);
        }
        else{
            return None;
        }
    }

    fn contains_point(&self, p : &Point) -> bool{
        if let Some(_x) = self.orient_point(p){
            return false;
        }
        let proj = p.project_to(self);
        if proj <= 1 && proj >= 0{
            return true;
        }
        false
    }
}

struct Triangle{v1: Point, v2: Point, v3: Point}

impl Triangle{
    fn centroid(&self) -> Point{
        //returns the center of mass
        Point{
            x : (Rational::from(&self.v1.x + &self.v2.x) + &self.v3.x)/3,
            y : (Rational::from(&self.v1.y + &self.v2.y) + &self.v3.y)/3,
        }
    }
}

trait GraphLike{
    fn as_edge(&self, e:&[usize;2]) -> Edge;
}


struct VisibilityGraph{
    vertices : Vec<Point>,
    edges : Vec<[usize;2]>,
    real_indices : Vec<usize>, //indexes the real edges inside edges. 
}

impl GraphLike for VisibilityGraph{
    fn as_edge(&self, e:&[usize;2]) -> Edge{
        Edge{
            v1: self.vertices[e[0]].clone(),
            v2: self.vertices[e[1]].clone(),
        }
    }
}


fn num_simp_ham(g : PartialCycle) -> Integer {
    //Calculates the number of crossing-free hamiltonian cycles
    //for a given list of points in general position. 

    let mut result = 0;
    for (i,v) in g.vertices.iter().enumerate(){
        if g.cycle.contains(i){continue}; //cycle stored as a list of indices
        let test_edge = g.vertices[i].subtract(g.cycle[-1]);
        
    }

    Integer::new()
}


// struct InducedTriangulation{
//     points : Vec<Point>,
//     //realIndices : Vec<usize>, //which of the points are "actually there"? 
//     n : usize, //The first n points are real, the rest are fake. 
//     triangles : Vec<[usize;3]>, //store the indices, due to ownership stuff
// }