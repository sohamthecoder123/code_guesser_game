use rand::Rng;

pub fn generate_random_code(n: usize, lower: isize, upper: isize) -> Vec<isize>{
    let mut result: Vec<isize> = Vec::new();

    for _i in 0..n{
        let ci: isize = rand::thread_rng().gen_range(lower..=upper);
        result.push(ci);
    }

    return result;
}

