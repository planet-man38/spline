fn main() {
    let mut currentn: u128 = 1;
    let mut previousn: u128 = 1;
    let mut iterations: u32 = 1;
    loop{
        if iterations > 100 {
            break;
        }
        let prevntemp = currentn;
        currentn += previousn;
        previousn = prevntemp;
        println!("{}", currentn);
        iterations += 1;
    }
  }
