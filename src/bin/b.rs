use proconio::input;

fn main() {
    input! {
        n: i32,
        x: [i64; n]
    }

    let manhattan = x.clone().into_iter().fold(0i64, |acc,z| acc + z.abs());
    println!("{}", manhattan);

    let euclid = x.clone().into_iter().fold(0i64, |acc,z| acc + z.pow(2));
    println!("{}", (euclid  as f64).sqrt());

    let chebi = x.clone().into_iter().fold(0i64, |acc, z| std::cmp::max(acc, z.abs()));
    println!("{}", chebi);


}
