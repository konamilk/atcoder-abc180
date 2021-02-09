use proconio::input;

fn main(){
    input! {
        mut x: u128,
        y: u128,
        a: u128,
        b: u128
    }

    let mut ans = 0u128;

    while a*x <= b+x && a*x < y {
        x = a*x;
        ans = ans+1;
    }

    ans += (y-x-1) / b;

    println!("{}", ans)
}

// -----------------------
// my submission
// -----------------------

// fn main() {
//     input! {
//         x: u128,
//         mut y: u128,
//         a: u128,
//         b: u128
//     }
//
//     y = y - 1;
//
//     let n1 = (((1 + b/x) as f64).ln() / (a as f64).ln()).floor() as u128;
//
//     let rest = y - x * a.pow(n1 as u32);
//
//     let n2 = rest / b;
//
//     println!("{}", n1 + n2)
// }
