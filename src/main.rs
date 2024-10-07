use::std::io;

fn main() {
    println!("We do some math");

    let mut damage = String::new();
    let mut c_damage = String::new();
    let mut c_rate = String::new();

    println!("Enter base damage: ");
    io::stdin().read_line(&mut damage).expect("failed to read line");

    println!("Enter critical damage: ");
    io::stdin().read_line(&mut c_damage).expect("failed to read line");

    println!("Enter critical rate: ");
    io::stdin().read_line(&mut c_rate).expect("failed to read line");

    let f_damage = damage.trim().parse().unwrap();
    let f_c_damage = c_damage.trim().parse().unwrap();
    let f_c_rate = c_rate.trim().parse().unwrap();
    crit_math(f_damage, f_c_damage, f_c_rate);
}

fn crit_math(damage: f32, c_damage: f32, c_rate: f32) {
    // with our variables b for base damage c for critical damage and r for critical rate critical damage can be expressed as b * [1 + r * c]
    // average damage increase for crits over a large number of hits is then expressed as
    // T = [1 - r] * N * b + r * N * b * [1 + c]
    // thus we can find our actual damage increase with the expression d = T/N

    println!("\nwith our variables b for base damage c for critical damage and r for critical rate critical damage can be expressed as b * [1 + r * c]");
    println!("average damage increase for crits over a large number of hits is then expressed as T = [1 - r] * N * b + r * N * b * [1 + c]");
    println!("thus we can find our actual damage increase with the expression d = T/N, with N being 10000");
    let b: f32 = damage;
    let c: f32 = c_damage;
    let r: f32 = c_rate;
    let n: f32 = 10000.0;
    println!("t = (1 - {}) * {} * {} + {} * {} * {} * (1 + {})", r, n, b, r, n, b, c);
    let t: f32 = (1.0 - r) * n * b + r * n * b * (1.0 + c);

    let mut average_damage: f32 = t/n;
    println!("Average damage, which is T/N, of a {} damage attack with a critical multipler of {} and a crit rate of {} is {}", b, c, r, average_damage);

    // this can be further simplified to the equation b * [1 + r * c], to prove this we do the following
    println!("this can be further simplified to the equation b * [1 + r * c], to prove this we do the following");
    println!("{} * (1 + {} * {})", b, r, c);
    average_damage = b * (1.0 + r * c);
    println!("Average damage of a {} damage attack with a critical multipler of {} and a crit rate of {} is {}", b, c, r, average_damage);
}