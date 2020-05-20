
/*use rand::Rng;

fn main() {
    let mut hina_score:u32=0;
    let mut another_person_score:u32=0;
    let mut dice_value:u32=0;
   

    println!("\n\nWhenever you want to exit the game, Press 'q'.");
    println!("First turn is doing by Hina.\n");

    while hina_score<100 && another_person_score<100 {

        dice_value=dice();
        hina_score=hina(dice_value,hina_score);
        println!("You are at {}",hina_score);
        println!("Press enter to continue...");   
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed"); 
        
        if input.trim()=="q"{ break;}
        if hina_score>=100{
            println!("\n\nHina won the game.");
            break;}
        println!("\n\nNow it's your turn.");


        dice_value=dice();
        another_person_score=other_person(dice_value,another_person_score);
        println!("You are at {}",another_person_score);
        println!("Press enter to continue...");
        input = String::new(); 
        std::io::stdin().read_line(&mut input).expect("Failed");
        
        if input.trim()=="q"{ break;}
        if another_person_score>=100 {
       println!("\n\nYou won the game."); 
            break;}
        println!("\n\nNow it's Hina's turn.");
    }
    
    
}
fn dice()->u32{
    let secret_number :u32= rand::thread_rng().gen_range(1, 6);
    println!("The number comes on dice is: {}", secret_number);
    return secret_number
}

fn hina(dice:u32, mut score:u32)->u32{
    score+=dice;
    if score==17 ||score==39 ||score==53 || score==79 ||score==81 ||score==94 ||score==99{
        println!("OH! OH! Snake.....at {}",score);
        score=0;
     }
    return score;
}

fn other_person(dice:u32,mut score:u32)->u32{
    score+=dice;
    if score==17 ||score==39 ||score==53 || score==79 ||score==81 ||score==94 ||score==99{
        println!("OH! OH! Snake.....at {}",score);
        score=0;
    }
    return score;
}*/


/*use std::{thread,time};                 //to use sleep
fn main(){
//let v:Vec<&str>="hina".matches("hina").collect();
//assert_eq!(v,["hina"] );
let mut num=0;
while num<3{
    println!("\n\nTo Access the Robot you must login first...
    If you want to exit during the program, Press Ctrl+c");

    println!("\n\nUsername : ");
    let mut say=String::new();
    std::io::stdin().read_line(&mut say).expect("Failed");

    let int=say.contains("hina");               //to match strings
    if int{
    println!("Password : ");
    say=String::new();
    std::io::stdin().read_line(&mut say).expect("Failed");
    let integer=say.contains("piaic123");
    if integer{
        robot_order();
    }
    else{println!("Your Password is Incorrect...");
    num+=1;
    }}
    else{println!("Your UserName is Incorrect...");
    num+=1}
    }
    println!("No More Try...");

}
fn robot_order(){
    println!("Hello, I am Robot. What do you want from me? Choose the following options...");
    
    println!("\n1. Turn on all lights of your home \n2. Turn off all lights of your home
    \n3. Show all camera's recording of your home.  \n4. Order a Chicken Pizza with Pepsi.
    \n5. Washed all dirty clothes. \n6. Read all my chats and reply to them using AI.");

    let mut option=String::new();
    std::io::stdin().read_line(&mut option).expect("Failed");
    let opt:u32=option.trim().parse::<u32>().unwrap();

    if opt==1{ println!("Command Accepted...\nPlease wait while we are performing your task.");
    thread::sleep(time::Duration::from_secs(5));            //robot take time to do task
    println!("All Lights have turned on. \nEnjoy! have a Great Day...");}

    else if opt==2{println!("Command Accepted...\nPlease wait while we are performing your task.");
    thread::sleep(time::Duration::from_secs(5));
    println!("All lights have turned off. \nEnjoy! have a Great Day...");}

    else if opt==3{println!("Command Accepted...\nPlease wait while we are performing your task.");
    thread::sleep(time::Duration::from_secs(5));
    println!("All recordings have been sent to your phone. \nEnjoy! have a Great Day...");}

    else if opt==4{println!("Command Accepted...\nPlease wait while we are performing your task.");
    thread::sleep(time::Duration::from_secs(5));
    println!("Here's your yummy food. \nEnjoy! have a Great Day...");}

    else if opt==5{println!("Command Accepted...\nPlease wait while we are performing your task.");
    thread::sleep(time::Duration::from_secs(5));
    println!("Clothes have been washed. \nEnjoy! have a Great Day...");}

    else if opt==6{println!("Command Accepted...\nPlease wait while we are performing your task.");
    thread::sleep(time::Duration::from_secs(5));
    println!("Replied to all chats. \nEnjoy! have a Great Day...");}
    
    else{println!("You must have to select from 1-6.");}

}*/

/*fn main(){
    println!("\n\nWhich table you need? Enter number...");
    let mut table_no=String::new();
    std::io::stdin().read_line(&mut table_no).expect("Failed");
    let tb_no:u32=table_no.trim().parse::<u32>().unwrap();

    let mut num:u32=1;
    while num<=20{
        println!("\t\t\t{} * {} = {}",tb_no,num,tb_no*num);
        num+=1;
    }    
}*/