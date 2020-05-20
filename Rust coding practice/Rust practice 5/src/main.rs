// fn main() {
//     println!("Press 1 for cooking Biryani. \nPress 2 for washing clothes" );
//     let mut input=String::new();
//     std::io::stdin().read_line(&mut input).expect("Failed");
//     let inp:u32=input.trim().parse::<u32>().unwrap();
//     if inp==1{ let mut order=cook(10, "Biryani".to_string());
//      println!("{}. You have performed this task.",order);}
//     else if inp==2{  let order=wash(15,"Surf".to_string());
//      println!("{} minutes taken. {}.",order.0,order.1);}
   
// }
// fn cook(no_of_hours:u32, dish:String)->String{
//     println!("{} hours are required to cook {}",no_of_hours,dish);
//     return "biryani Is cooked.".to_string()
// }
// fn wash(no_of_clothes:u32,surf_name:String)->(u32,String)
// {   let clothes=no_of_clothes+5;
//     println!("{} clothes are washed by {}",clothes,surf_name);
//     let  minutes=45;
//     let  msg=String::from("Clothes washed.");
//     return (minutes,msg);
// }
/////////////////////////////////question 2
fn main(){
    println!("Enter a number where you want to print prime numbers.");
    let mut number=String::new();
    std::io::stdin().read_line(&mut number).expect("FAiled");
    let no=number.trim().parse::<u32>().unwrap();

    for i in 1..no{
        let mut count=0;
        let mut j=1;
       while j<=i {
          
         
            if (i%j)==0{
                count=count+1;
                
            }
              j=j+1;
            }
           if count==2{
               println!("{} is a prime number",i);
           }
       }
}
