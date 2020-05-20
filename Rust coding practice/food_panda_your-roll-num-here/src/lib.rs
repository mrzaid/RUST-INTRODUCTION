// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
 
pub mod dealy_meal{
    pub mod dinner{
        println!("This is dinner module");
    }
    pub mod lunch{
        pub fn fun1(){
            println!("This is the fun1 of lunch.");
        }
        pub fn fun2(){
            println!("This is the fun2 of lunch.");
        }
    }
    pub mod breakfast{
        pub fn fun3(){
            println!("This is the fun3 of breakfast.");
        }
    }
}
