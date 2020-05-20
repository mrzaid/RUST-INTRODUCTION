/*enum Direction{
        forward,
        backward,
        left,
        right,
    }
fn main() {
check_direction(Direction::left);

}
fn check_direction(dir:Direction){
    match dir{
        Direction::forward=>println!("Vehicle is moved forward."),
        Direction::backward=>println!("Vehicle is moved backward."),
        Direction::left=>println!("Vehicle is moved left."),
        Direction::right=>println!("Vehicle is moved right."),
    }
}*/

fn main(){
    let some=Some(8);
    print!("The value of Some is: {:?}",add_one(some));
    let no:Option<i32>=None;
    print!("The value of no is: {:#?}",add_one(no));
}

fn add_one(option:Option<i32>)->Option<i32>{
    match option{
        None=>None,
        Some(i)=>Some(i+1),
    }
}

