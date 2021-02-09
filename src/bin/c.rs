use proconio::input;

fn main() {
    input! {
        n: i64
    }

    let mut yaku = vec![];

    for i in 1..=(n as f64).sqrt() as i64 {
        if n % i == 0{
            yaku.push(i);
            yaku.push(n/i);
        }
    }

    yaku.sort();
    yaku.dedup();

    for j in yaku{
        println!("{}", j)
    }
}
