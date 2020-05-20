fn main(){
    //******************question 1*************
// let name="Hina";
// let email="hinakhadim2002@gmail.com";
// let id="IOT048420";
// let age=17;
// println!("Name: {}  \nEmail: {} \nId: {}  \nAge: {}",name,email,id,age);
//     //******************question 2***************
// let var=10;
// let var=var+1;
// let var=var*6;
// print!("Value of var: {}",var);
//     //******************question 3******************
// let num1:i32 =1000;
// let num2:i32 =500;
// println!("{} + {} = {}",num1,num2,num1+num2);
// println!("{} - {} = {}",num1,num2,num1-num2);
// println!("{} * {} = {}",num1,num2,num1*num2);
// println!("{} / {} = {}",num1,num2,num1/num2);
    //********************question 4****************
// let data=("Hina","hinakhadim2002@gmail.com","iot048420",17);
// let name=data.0;
// let email=data.1;
// let id=data.2;
// let age=data.3;
// println!("Name: {}   \nEmail: {}    \nId: {}    \nAge: {}",name,email,id,age);
//     //*********************question 5********************
let friend:[& str;5]=["Hina","Kainat","Khizra","VAzeema","Tooba"];
let age:[u8;5]=[17,18,19,20,18];
println!("Friend Name   : {}     -   Age:   {}",friend[0],age[0]);
println!("Friend Name   : {}   -   Age:   {}",friend[1],age[1]);
println!("Friend Name   : {}   -   Age:   {}",friend[2],age[2]);
println!("Friend Name   : {}  -   Age:   {}",friend[3],age[3]);
println!("Friend Name   : {}    -   Age:   {}",friend[4],age[4]);
}