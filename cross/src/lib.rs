#[no_mangle]
pub extern "C" fn plus_two(n: i32) -> i32 {
    n + 2
}

extern "C" {
    pub fn add_one(
        i: cty::c_int
    ) -> cty::c_int;
}

#[no_mangle]
pub extern "C" fn plus_three(n: i32) -> i32 {
    let n =  unsafe { add_one(n + 2) };

    let m = n + 1; 
    m
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_two() {
        assert_eq!(plus_two(0), 2);
        assert_eq!(plus_two(5), 7);
        assert_eq!(plus_two(-3), -1);
    }
}
