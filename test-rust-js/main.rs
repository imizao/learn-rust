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
    // i32 时 设值100 运行到46的时候，打印会溢出报错 （i32 46 耗时16.0230357s）
    // （i64 46 耗时18.3018556s）
    //  thread 'main' panicked at 'attempt to add with overflow', src\main.rs:39:12
    //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // js 值为50的时候，运行时间abc: 3:19.461 (m:ss.mmm) 着实惊呆了，因为同为50， rust运行时间为 循环的耗时: 129.8715647s
    let start = Instant::now();
    for i in 1..50 {
        let a = cui(i);
        println!("{}",a)
    }
    let duration = start.elapsed();
    println!("循环的耗时: {:?}", duration);
}

fn cui(n:i64) -> i64{
    if n == 1 {return 1;}
    if n == 2 {return 2;}

    return cui(n-1) + cui(n-2)
}