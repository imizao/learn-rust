fn large_n<T: std::cmp::PartialOrd>(a:T,b:T) ->T {
    if a > b {
        a
    }else {
        b
    }
}

fn main() {
    let a_num = 10;
    let b_num = 20;
    let p_num = large_n(a_num,b_num);
    println!("{}",p_num);
}
