fn main() {
    println!("Enter the Actual Price : ");
    let  mut a_price= String::new();
    std::io::stdin().read_line(&mut a_price).expect("Failed");
    let a_price=a_price.trim().parse::<u64>().unwrap();
    println!("Enter The Discounted Price : ");
    let mut dis_price= String::new();
    std::io::stdin().read_line(&mut dis_price).expect("Failed");
    let dis_price=dis_price.trim().parse::<u64>().unwrap();
    println!("Enter Customer Payment Amount : ");
    let mut pay= String::new();
    std::io::stdin().read_line(&mut pay).expect("Failed");
    let pay=pay.trim().parse::<u64>().unwrap();
    let (dis,percent,balance)=calculate(a_price,dis_price,pay);

        print!("Discount = {}\nPercent Discount = {}\nBalance = {}",dis,percent,balance );
    if percent<=10.0{
        println!("HURRAH! AZADI OFFER");
    }
    else if percent>=10.0 && percent<=20.0{
        println!("HURRAH! EID OFFER");
    }
    else if percent>20.0{
        println!("HURRAH! CLEARANCE SALE");
    }

}
fn calculate(actual:u64,discounted:u64,pay:u64)->(u64,f64,u64){
    let dis_amount:u64=actual-discounted;
    let percent:f64=dis_amount as f64 *100.0/actual as f64;
    let balance:u64=pay-discounted;
    (dis_amount,percent,balance)
    }

