pub fn is_leap_year(year: i32) -> bool {
//    can use variable
//    let leap_year: bool;
    if (year % 4 != 0 ) | ((year % 100 == 0) && (year % 400 != 0)) {
        false
//        leap_year = false;
    }
    else {
        true
//        leap_year = true;
    }
//    leap_year
}
