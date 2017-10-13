use std::string::ToString;

pub fn raindrops(n: usize) -> String {
    let mut result: String = String::from("") ;
    let mut rem: usize;
    let n_string: String = n.to_string();

    for divisor in 1..n+1 {
        rem = n % divisor;
        if divisor == 3 && rem == 0 {result.push_str("Pling"); }
        else if divisor == 5 && rem == 0 { result.push_str("Plang"); }
        else if divisor == 7 && rem == 0 { result.push_str("Plong"); }
    }
    if result == "" {
         result.push_str(&n_string);
         result
    }
    else {
        result
    }
}
