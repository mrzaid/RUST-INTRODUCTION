#[derive(Debug)]
struct Student{
    name: String,
    grade: String,
    age: u8,
    percentage: f32,
}

impl Student{
    fn new()-> Student{
        let stu1=Student{
            name: String::from("Hina"),
            grade: String::from("A+"),
            age: 17,
            percentage: 80.92,
        };
        stu1
    }
    fn percent(&self){
        println!("{}% is your Result.",self.percentage);
    }
}

fn main() {
    let student1=Student::new();
    println!("Student instance : \n{:#?} ",student1);
    student1.percent();
}
