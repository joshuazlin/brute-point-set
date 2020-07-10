/*
Hopefully implements arbitrary precision rational geometry in 2D.
*/

use rug::{Rational, Integer};

/*
Here are the global functions
*/
fn xor_option(a : Option<bool>, b : Option<bool>) -> Option<bool>{
    match (a,b){
        (Some(x),Some(y)) => Some(x^y),
        _ => None,
    }
}

/*Two by two matrices*/
struct Matrix2{a : [[Rational;2];2]}

impl Matrix2{
    fn det(&self) -> Rational{
        self.a[0][0]*self.a[1][1] - self.a[0][1]*self.a[1][0]
    }

    fn scale(&self, x : &Rational) -> Matrix2{
        Matrix2{
            a : [[self.a[0][0]*x,self.a[0][1]*x],
                 [self.a[1][0]*x,self.a[1][1]*x]]
        }
    }

    fn inverse(&self) -> Option<Matrix2>{
        let d = self.det();
        if d == Rational::from(0){return None;}
        let inv = Matrix2{
            a : [[self.a[1][1],-self.a[1][0]],
                 [-self.a[0][1],self.a[0][0]]]
        };
        inv.scale(&(Rational::from(1)/d));
        return Some(inv)
    }

    fn apply(&self, v : Vect2D) -> Vect2D{
        Vect2D{
            x : self.a[0][0]*v.x+self.a[0][1]*v.y,
            y : self.a[1][0]*v.x+self.a[1][1]*v.y,
        }
    }
}

/*
A point struct, lives in 2D. 
*/
#[derive(Clone)]
pub struct Point{x: Rational, y : Rational}

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


/*
A 2D-vector struct. 
*/
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

    fn as_point(&self) -> Point{
        Point{
            x : self.x,
            y : self.y,
        }
    }
}

/*
An Edge struct. 
*/
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

    fn line_contains_point(&self, p : &Point) -> bool{
        //Does the line which extends from this edge contain the point? 
        match self.orient_point(p){
            None => true,
            Some(x) => false,
        }
    }

    fn intersect(&self, e : &Edge) -> bool{
        let o1 = self.orient_point(&e.v1);
        let o2 = self.orient_point(&e.v2);
        let o = xor_option(o1,o2);
        
        let u1 = e.orient_point(&self.v1);
        let u2 = e.orient_point(&self.v2);
        let u = xor_option(u1,u2);

        if o == Some(false){return false;}
        if u == Some(false){return false;}
        if xor_option(o,u) == Some(false){return true;}

        if o1 == None{if self.contains_point(&e.v1){return true;}}
        if o2 == None{if self.contains_point(&e.v2){return true;}}
        if u1 == None{if e.contains_point(&self.v1){return true;}}
        if u2 == None{if e.contains_point(&self.v2){return true;}}

        false
    }

    fn line_equation(&self) -> [Rational;3]{
        //a x + b y + c = 0, 
        [self.v2.x-self.v1.x,
         self.v1.y-self.v2.y,
         self.v1.x*(self.v1.x-self.v2.x)+self.v1.y*(self.v2.y-self.v1.y)]
    }

    fn intersection_point(&self, e: &Edge) -> Option<Point>{
        //returns the intersection point of the two *lines* 
        //I'd feel bad about writing a line struct though... it feels wrong. 
        let l1 = self.line_equation();
        let l2 = e.line_equation();
        let M = Matrix2{
            a : [[l1[0],l1[1]],
                 [l2[0],l2[1]]]
        }.inverse();
        let v = Vect2D{
            x : l1[2],
            y : l2[2],
        };
        match M{
            None => None,
            Some(N) => Some(N.apply(v).as_point()),
        }
    }
}

/*
A Triangle struct. 
*/
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


/*
A set of points struct
*/
pub struct PointSet{
    pub vertices : Vec<Point>,
}

impl PointSet{
    fn as_edge(&self, e:&[usize;2]) -> Edge{
        Edge{
            v1: self.vertices[e[0]].clone(),
            v2: self.vertices[e[1]].clone(),
        }
    }
}

/*
A struct for graphs?
*/
struct Graph{
    pub ps : PointSet,
    pub edges : Vec<[usize;2]>,
}

impl Graph{
    //Somehow I need to pass things up or something? this is dumb 
    fn vertices(&self) -> Vec<Point>{
        self.ps.vertices
    }
}

/*
A struct for Visibility Graphs??
*/
pub struct VisibilityGraph{
    pub g : Graph,
    pub vis_edges : Vec<[usize;2]>,
}

impl VisibilityGraph{

    pub fn vertices(&self) -> Vec<Point>{
        self.g.vertices()
    }

    pub fn edges(&self) -> Vec<[usize;2]>{
        self.g.edges
    }

    pub fn add_edge(&self, e:[usize;2]){
        //Add this edge as a real edge. 
        self.g.edges.push(e);
        let mut to_remove = vec![];
        for (i,f) in self.vis_edges.iter().enumerate(){
            if self.g.ps.as_edge(&e).intersect(&self.g.ps.as_edge(f)){
                to_remove.push(i);
            }
        }
        to_remove.reverse();
        for i in to_remove.iter(){
            self.vis_edges.remove(*i);
        }
    }

}






// /*
// Here are a bunch of Traits
// */
// trait PointSet{
//     fn vertices(&self) -> Vec<Point>;
//     fn as_edge(&self, e:&[usize;2]) -> Edge;
//     fn is_generic(&self, v : &Point) -> bool; //is v generic with respect to self. 
// }

// trait VisibilityGraphLike : PointSet{
//     fn visible(&self,i : usize) -> Vec<usize>;
//     fn add_edge(&mut self,e : [usize;2]);
//     fn add_vertex(&mut self, v : Point);
// }

// /*
// Here is the data structure we are working with mainly, which is just a visibility graph
// but it remembers a "cycle" that it's trying to build up. 

// I think the triangulation should be a struct that floats on top. YOOOO that actually makes some sense I think
// */
// #[derive(Clone)]
// struct PartialCycleGraph{
//     vertices : Vec<Point>,
//     edges : Vec<[usize;2]>,
//     visibility_edges : Vec<[usize;2]>,
//     cycle : Vec<usize>,
// }

// impl PartialCycleGraph{
//     fn new() -> PartialCycleGraph{
//         PartialCycleGraph{
//             vertices : vec![],
//             edges: vec![],
//             visibility_edges : vec![],
//             cycle : vec![],
//         }
//     }
// }

// impl PointSet for PartialCycleGraph{
//     fn vertices(&self) -> Vec<Point>{
//         self.vertices
//     }

//     fn as_edge(&self, e:&[usize;2]) -> Edge{
//         Edge{
//             v1: self.vertices[e[0]].clone(),
//             v2: self.vertices[e[1]].clone(),
//         }
//     }

//     fn is_generic(&self, v : &Point) -> bool{
//         //Is it enough to demand that it isn't on any of the existing lines? who knows
//         //well for now, that's what we are going to do

//         // !(0..self.vertices.len()).map(|i| 
//         //     (i+1..self.vertices.len()).map(|j| 
//         //         self.as_edge(&[i,j]).line_contains_point(v)
//         //     ).collect::<Vec<bool>>().contains(&true)
//         // ).collect::<Vec<bool>>().contains(&true)

//         //Also need to check that it isn't parallel to anything... sad emoji reacts only
//         //Really, we just need to check that every new edge intersects every other in exactly one place? 
//     }
// }

// impl VisibilityGraphLike for PartialCycleGraph{
//     fn visible(&self,i:usize) -> Vec<usize>{
//         //Returns all the vertices that are visible from the vertex i. 
//         let mut to_return = vec![];
//         for e in &self.visibility_edges{
//             match e{
//                 [i,x] => to_return.push(*x),
//                 [x,i] => to_return.push(*x),
//             }
//         }
//         to_return
//     }

//     fn add_edge(&mut self,e : [usize;2]){
//         //Add this edge as a real edge. 
//         self.edges.push(e);
//         let mut to_remove = vec![];
//         for (i,f) in self.visibility_edges.iter().enumerate(){
//             if self.as_edge(&e).intersect(&self.as_edge(f)){
//                 to_remove.push(i);
//             }
//         }
//         to_remove.reverse();
//         for i in to_remove.iter(){
//             self.visibility_edges.remove(*i);
//         }
//     }

//     fn add_vertex(&mut self, v : Point){
//         self.vertices.push(v);
//         let i = self.vertices.len()-1;
//         for j in 0..self.vertices.len(){
//             let temp_edge = self.as_edge(&[i,j]);
//             if !self.edges.iter().map(|e|self.as_edge(e).intersect(&temp_edge)).collect::<Vec<bool>>().contains(&true){
//                 self.visibility_edges.push([i,j]);
//             }
//         }
//     }
// }

// /*
// here's a triangulation struct that floats on top. It is the max-triangulation
// induced by the points of g; (so connect all the possible edges, and the 
// intersections are considered points too). We don't care about the faces on the 
// outside of g, if that makes sense. 
// */
// struct MaxTriangulation{
//     g : Box<dyn PointSet>, //bruh... I have no idea what this means
//     intersections : Vec<Point>, //includes all the points of g?
//     triangles : Vec<[usize;3]>, //indexes the intersections I guess
// }

// impl MaxTriangulation{
//     fn add_vertex(&self, v : Point, i : Option<usize>){
//         //gonna assume for now that v lives in triangle i because I'm lazy
//         if let Some(j) = i{
//             let t = self.triangles[j]; //Do I have to copy stuff? brain slow
//             self.triangles.remove(j);
//             self.g.vertices().push(v);
//             let l = self.g.vertices().len()-1;

//             self.triangles.push([t[0],t[1],l]);
//             self.triangles.push([t[1],t[2],l]);
//             self.triangles.push([t[2],t[0],l]);
//         }
//     }

//     fn as_triangle(&self, i : [usize;3]) -> Triangle{
//         Triangle{
//             v1: self.g.vertices()[i[0]],
//             v2: self.g.vertices()[i[1]],
//             v3: self.g.vertices()[i[2]],
//         }
//     }

//     fn find_generic_point(&self, i:usize){
//         //For triangle i, finds a generic point inside the triangle. 
//         let t = self.triangles[i];
//         let tr = self.as_triangle(t);
//     }
// }





