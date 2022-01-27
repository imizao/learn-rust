use std::fmt;
#[derive(Debug)]
struct MinMax(f64,f64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({}, {})",self.0,self.1)
    }
}
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x:{},y:{}",self.x,self.y)
    }
}

use std::time::{Instant};

fn main() {
    let start = Instant::now();
    for i in 1..40 {
        let a = cui(i);
        println!("{}",a)
    }
    let duration = start.elapsed();
    println!("循环的耗时: {:?}", duration);
}

fn cui(n:i32) -> i32{
    if n == 1 {return 1;}
    if n == 2 {return 2;}

    return cui(n-1) + cui(n-2)
}