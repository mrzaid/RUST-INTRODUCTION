fn main(){
// let num=5;
// let rem=num%2;
// println!("Remainder is : {}",rem);
// println!("Binary Number is : {:b}",num);
// println!("Binary Number is : {:x}",num+6); //hexadecimal
// let hex=0xfff;// 4095  &  fffff=255
// println!("Binary Number is : {}",hex); //
// let bin=0b101010;
// println!("Binary Number is : {}",bin);

// let age=33;
println!("Bakra Eid Mubarak");
//let arg="Hello";
//qurbani(0,arg); //******************************calling function is expression
//qurbani(7,"0");
//beef_karachi(12,1,8);
//let y=6;   is a statement 
//************************************call macro  e.g. println!()  is a expression ; // println!(); is a statement
//let x=(let y=6);             error

//***********************************************************************************************
// let mut x = 5;
// let y = {
//      x = x+3;
//     println!("The value of y  inside x is: {}", x);
//     x + 1
// };
// println!("The value of x outside is: {}", x);
// println!("The value of y is: {}", y);
// let x=five();
// print!("{}",x);
/////////////88888888888888888888888888888888888888888888888888888888888888888888888
let h=square(10);
println!("Square= {}",h);
// println!("Beef Gift {} kg",h.0);
// println!("Beef Gift {} kg",h.1);
// println!("Beef Gift {} kg",h.2);
//println!("Beef Gift for sir Areeb {} kg",zia());
}
// fn zia()->(i32,f64,u8) {
//     let beef:u8=5;
//     println!("beef for distribution per person {} kg",beef);
//     let tup=(400,5.5,1);
//     tup    
// }
fn square(x:i32)->i32{
    x*x}
//*********************we can return two values at a time by using tuple
// fn five()->String 
// {  "hina khadim".to_string() }


// fn qurbani(num2:i32,num1:& str)->i32{  //function definition is also a statement
//     println!("{}",num1);
//     println!("{}",num2);
//     println!("Mutton");
//     println!("Chicken");
//     println!("Karahi");
//     println!("Nehari");
//     println!("Siri Payee");
// }
// fn beef_karachi(beef:u8,chicken:u8,mutton:u8){
// print!("Beef Karahi : {} kg",beef);
// print!("\tChicken Karahi : {} kg",chicken);
// print!("\tMutton Karahi : {} kg",mutton);
// }

// which returns a value is a expression
//which do not return is a statement.  ex:    6
// let is a statement.