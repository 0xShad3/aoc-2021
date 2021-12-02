struct Operation {
    dir: String,
    val: i64,
}

fn stage1(ops: &Vec<Operation>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    for op in ops{
        match op.dir.as_ref(){
            "forward" => x += op.val,
            "down" => y += op.val,
            "up" => y -= op.val,
            _ => println!("An error occured with these values {} {} ", op.val, op.dir)
        }
    }
    return x * y;
}

fn stage2(ops: &Vec<Operation>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for op in ops{
        match op.dir.as_ref(){
            "forward" => {
                    x += op.val;
                    y += z * op.val;
                }
            "down" => z += op.val,
            "up" => z -= op.val,
            _ => println!("An error occured with these values {} {} ", op.val, op.dir)
        }
    }
    return x * y;
}
fn main() {
    let input_str = std::fs::read_to_string("./rsrc/input.txt")
                            .expect("An error occured");
    let input: Vec<&str> = input_str
                          .trim()
                          .split("\n")
                          .collect();
    let mut ops : Vec<Operation> = Vec::new();   
    for st in input{
        let elems : Vec<&str> = st.split(" ").collect::<Vec<&str>>();
        ops.push(Operation {
                 dir: String::from(elems[0]),
                 val: elems[1].parse::<i64>().unwrap()
                });
    }
    println!("Solution 1 {}", stage1(&ops));
    println!("Solution 2 {}", stage2(&ops));
}
