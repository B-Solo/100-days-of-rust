fn main() {
    println!("{}", calc_age(20));
}


fn calc_age(age: i64) -> i64{
    365 * age
}