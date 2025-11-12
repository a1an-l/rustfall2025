use std::fmt::Display;


trait ShowInfo {
    fn show_info(&self) -> String;
}


struct UndergradStudent {
    name: String,
    major: String,
    gpa: f32,
}

impl ShowInfo for UndergradStudent {
    fn show_info(&self) -> String {
        format!(
            "Undergrad: {}, Major: {}, GPA: {:.2}",
            self.name, self.major, self.gpa
        )
    }
}


struct GradStudent {
    name: String,
    major: String,
    gpa: f32,
    thesis_title: String,
}

impl ShowInfo for GradStudent {
    fn show_info(&self) -> String {
        format!(
            "Graduate: {}, Major: {}, GPA: {:.2}, Thesis: {}",
            self.name, self.major, self.gpa, self.thesis_title
        )
    }
}


struct Enrollment<T: ShowInfo> {
    students: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Enrollment { students: Vec::new() }
    }

    fn add(&mut self, student: T) {
        self.students.push(student);
    }

    fn display_all(&self)
        {
        for s in &self.students {
            println!("{}", s.show_info());
        }
    }
}



struct MixedEnrollment {
    students: Vec<Box<dyn ShowInfo>>,
}

impl MixedEnrollment {
    fn new() -> Self {
        MixedEnrollment { students: Vec::new() }
    }

    fn add<T: ShowInfo + 'static>(&mut self, student: T) {
        self.students.push(Box::new(student));
    }

    fn display_all(&self) {
        for s in &self.students {
            println!("{}", s.show_info());
        }
    }
}


fn main() {
    let undergrad1 = UndergradStudent {
        name: "Alice".to_string(),
        major: "Computer Science".to_string(),
        gpa: 3.8,
    };

    let grad1 = GradStudent {
        name: "Bob".to_string(),
        major: "Electrical Engineering".to_string(),
        gpa: 3.9,
        thesis_title: "Machine Learning for Edge Devices".to_string(),
    };

    println!("Separate Enrollment");
    let mut undergrad_enroll = Enrollment::<UndergradStudent>::new();
    let mut grad_enroll = Enrollment::<GradStudent>::new();

    undergrad_enroll.add(undergrad1);
    grad_enroll.add(grad1);

    undergrad_enroll.display_all();
    grad_enroll.display_all();

    println!("\nMixed Enrollment");
    let mut mixed_enroll = MixedEnrollment::new();

    mixed_enroll.add(UndergradStudent {
        name: "Charlie".to_string(),
        major: "Mathematics".to_string(),
        gpa: 3.7,
    });

    mixed_enroll.add(GradStudent {
        name: "Diana".to_string(),
        major: "Physics".to_string(),
        gpa: 4.0,
        thesis_title: "Quantum Entanglement Applications".to_string(),
    });

    mixed_enroll.display_all();
}
