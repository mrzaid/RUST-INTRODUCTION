/*fn main() {
    let mut shopping_cart=Vec::new();
    shopping_cart.push("orange");
    shopping_cart.push("banana");
    shopping_cart.push("apple");
    println!("The length of shopping_cart is : {}",&shopping_cart.len());
    for element in 0..shopping_cart.len(){
        println!("{}",&shopping_cart[element]);
    }
    let mut prices=vec![20,30,40];
    for price in &mut prices{
        *price+=5;
        println!("{}",price);
    }
    let first=&shopping_cart[0]; 
    match shopping_cart.get(0){
        Some(first)=> println!("First element of cart is {}.",first ),
        None=> println!("There is no element."),
    }
}*/
fn main(){
    let mut japanese=String::new();
    japanese.push_str("안녕하세요");
    let mut arabic=String::from("علیكم السالم");
    let mut jap_ara=format!("{} {}",japanese,arabic);
    jap_ara.push('!');
    println!("{}",jap_ara);
    for letter in arabic.chars(){
        println!("{}",letter);
    }
    for memory in arabic.bytes(){
        println!("{}",memory);
    }
}
/*use std::collections::HashMap;
fn main(){
    let mut Team_scores=HashMap::new();
    Team_scores.insert(String::from("Pakistan"),250);
    Team_scores.insert(String::from("New Zealand"),309);
    Team_scores.insert(String::from("England"),224);
    Team_scores.entry(String::from("Egypt")).or_insert(450);
    Team_scores.entry(String::from("Pakistan")).or_insert(1000);
    for (key,value) in &Team_scores{
        println!("{} = {}",key,value);
    }
}*/
