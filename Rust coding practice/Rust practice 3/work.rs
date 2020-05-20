use std::io;  // to read string
//*******************************************question 1***********8
fn main(){
    println!("Enter a number of your choice: ");
   let mut input=String::new();
   io::stdin().read_line(&mut input).expect("could not read line");
   let _i:i32=input.trim().parse::<i32>().unwrap();
   _decide(_i);
}
   
//     let mut input = String::new();
// io::stdin().read_line(&mut input).unwrap();
// let n: i32 = input.trim().parse().unwrap();
// println!("{}",n);
// fn get_input() -> String {
//     let mut buffer = String::new();
//     std::io::stdin().read_line(&mut buffer).expect("Failed");
//     buffer
// }

// let _x = get_input().trim().parse::<i64>().unwrap();
fn _decide(_num:i32){
    if _num > 0 && _num!=50{
        println!("{} is positive number",_num);
    }
    else if _num < 0{
        println!("{} is negative number",_num);
    }
    else if _num==50{
        println!("{} is equal to 50.",_num);
    }
}


//******************************************question 2********
// fn main(){
//     println!("Enter an integer of your choice: ");
//    let mut input=String::new();
//    io::stdin().read_line(&mut input).expect("could not read line");
//    let _int:i32=input.trim().parse::<i32>().unwrap();

//    println!("Enter a float of your choice: ");
//    let mut input1=String::new();
//    io::stdin().read_line(&mut input1).expect("could not read line");
//    let _float:f32=input1.trim().parse::<f32>().unwrap();

//    let _bool:bool=true;

//    println!("Enter Your Name: ");
//    let mut _string=String::new();
//    io::stdin().read_line(&mut _string).expect("could not read line");

//    _print(_int,_float,_bool,_string);
   

// }
// fn _print(_int:i32,_float:f32,_bool:bool,_string:String){
//     println!("{} is an integer. ",_int);
//     println!("{} is an float. ",_float);
//     println!("{} is an bool value. ",_bool);
//     print!("{} is my Name. ",_string);
//     }

//******************************************question 3****************

// fn main(){
//      println!("Enter a number of your choice: ");
//     let mut input=String::new();
//     io::stdin().read_line(&mut input).expect("could not read line");
//     let _i:i32=input.trim().parse::<i32>().unwrap();
//     let tup=_square(_i);
//     println!("{} is the number itself.",tup.0);
//     println!("{} is the square of number.",tup.1);
// }
// fn _square(_num:i32)->(i32,i32){
//     let tup=(_num,_num*_num);
//     tup}

//**************************************question 4**********

// fn main()
// {
//     let english:f32=67.0;
//     let math:f32=90.5;
//     let percent:f32=_percentage(english,math);
//     println!("Percentage is {}%",percent);
//     if percent>=70.0{
//         println!("Passed with {}%",percent);
//     }
//     else{
//         println!("Fail");
//     }
//     }
// fn _percentage(eng:f32,math:f32)-> f32{
//     let obtain_marks:f32=eng+math;
//     println!("Obtained Marks : {}",obtain_marks);
//     let _percent:f32=(obtain_marks/200.0)*100.0;
//     _percent
// }
