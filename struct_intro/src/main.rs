struct Student{
    name: String,
    major: String,
}

impl Student{
    fn new( n: String, m: String) ->Student {
        Self{
            name: n,
            major: m,
        }
    }

    fn set_major(&mut self, new_major: String){
        self.major= new_major;
    }

    fn get_major( &self) -> &String {
        return &self.major;
    }

}

fn main() {
    let mut student1= Student:: new("Alan".to_string(), "Computer Science".to_string());
    println!("Major: {}", student1.get_major());
    student1.set_major("Cyber Security".to_string());
    println!("Major: {}", student1.get_major());

}
