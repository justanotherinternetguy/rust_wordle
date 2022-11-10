use rand::Rng;


fn main() {
    let words = ["cigar","rebut","sissy","humph","awake","blush","focal","evade","naval","serve","heath","dwarf","model","karma","stink","grade","quiet","bench","abate","feign","major","death","fresh","crust","stool","colon","abase","marry","react","batty"];
    let mut rng = rand::thread_rng();

    let ind: u8 = rng.gen();
    println!("{}", ind);

}
