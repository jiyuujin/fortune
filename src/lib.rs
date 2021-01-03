#[no_mangle]
pub extern "C" fn age(cy: i32, by: i32) -> i32 {
   cy - by
}

#[no_mangle]
pub extern "C" fn fate_number(y: i32, m: i32, d: i32) -> i32 {
    let mut sum = 0;
    for i in y.to_string().chars() {
        let tmp = (i.to_string()).parse::<i32>().unwrap();
        sum += tmp;
    }
    for i in m.to_string().chars() {
        let tmp = (i.to_string()).parse::<i32>().unwrap();
        sum += tmp;
    }
    for i in d.to_string().chars() {
        let tmp = (i.to_string()).parse::<i32>().unwrap();
        sum += tmp;
    }
    return sum
}
