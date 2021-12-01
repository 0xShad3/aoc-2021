
fn shift_push(window:&mut Vec<i64>, elem: i64) {
    if window.len() <= 3{
        window.remove(0);
        window.push(elem);   
    }
}

fn windows_sum(window:&mut Vec<i64>) -> i64{
    let mut sum : i64 = 0;
    for i in 0..3{
        sum += window[i]; 
    }
    return sum;
}
fn stage1(input: Vec<i64>) -> u32{
    let mut incr_counter = 0;
    let input_len = input.len();

    for i in 1..input_len{
        if input[i] > input[i-1]{
            incr_counter+=1;
        }
    }    
    return incr_counter;
}

fn stage2 (input: Vec<i64>) -> i64 {
    let mut inc_counter: i64 = 0;
    let mut temp: i64 = 0;
    let input_len: usize = input.len();
    let mut window: Vec<i64> = Vec::with_capacity(3);
    window.extend([input[0], input[1], input[2]]);

    for i in 3..input_len {
        if temp < windows_sum(&mut window){
            inc_counter+=1;
        }
        temp = windows_sum(&mut window);
        shift_push(&mut window, input[i]);
    }
    return inc_counter;
}
fn main() {
    let input_str = std::fs::read_to_string("../rsrc/input.txt")
                            .expect("An error occured");
    let input: Vec<i64> = input_str
                          .trim()
                          .split("\n")
                          .map(|s| s.parse::<i64>().unwrap())
                          .collect();
    
    println!("Solution 1 : {:?}", stage1(input.clone()));
    println!("Solution 2 : {:?}", stage2(input.clone()));

}
