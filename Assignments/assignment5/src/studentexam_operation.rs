//Creating Student of structure type
//
//#Argument
//name, roll_no, Score as structure for score of each subject,department, school
//
//Return creating Student_data,Average calculation,guess mark and comparing the score

pub struct Student {
    pub name: String,
    pub roll_no: i32,
    pub(crate) scoreofeachsubject: Score,
    pub department: String,
    pub school: String,
}
pub struct Score {
    pub(crate) hindi: i32,
    pub(crate) english: i32,
    pub(crate) maths: i32,
    pub(crate) science: i32,
}

//Action implementation on Student dataset
impl Student {
    pub fn new(scoreofeachsubject: &Score) -> Student {
        Student {
            name: "Dheeraj".to_string(),
            roll_no: 25,
            scoreofeachsubject: Score {
                hindi: scoreofeachsubject.hindi,
                english: scoreofeachsubject.english,
                maths: scoreofeachsubject.maths,
                science: scoreofeachsubject.science,
            },
            department: "IT".to_string(),
            school: "IMS Engineering college".to_string(),
        }
    }
    //Calculating average of marks
    pub fn average_calculator(&self) -> f32 {
        let result: f32 = ((self.scoreofeachsubject.maths
            + self.scoreofeachsubject.science
            + self.scoreofeachsubject.english
            + self.scoreofeachsubject.hindi)
            / 4) as f32;
        result
    }
    //Performing action to added extra marks below 35
    pub fn guess_mark(&mut self) {
        if self.scoreofeachsubject.hindi < 35 {
            self.scoreofeachsubject.hindi += 35 - self.scoreofeachsubject.hindi
        } else if self.scoreofeachsubject.english < 35 {
            self.scoreofeachsubject.english += 35 - self.scoreofeachsubject.english
        } else if self.scoreofeachsubject.science < 35 {
            self.scoreofeachsubject.science += 35 - self.scoreofeachsubject.science
        } else if self.scoreofeachsubject.maths < 35 {
            self.scoreofeachsubject.maths += 35 - self.scoreofeachsubject.maths
        }
    }
    //Printing&Comparing marks of two different student
    pub fn compare_student(&self, another_student: &Score) {
        println!(
            "For HindiDifference {}",
            (self.scoreofeachsubject.hindi - another_student.hindi).abs()
        );
        println!(
            "For english_difference {}",
            (self.scoreofeachsubject.english - another_student.english).abs()
        );
        println!(
            "For maths_difference {}",
            (self.scoreofeachsubject.maths - another_student.maths).abs()
        );
        println!(
            "For science_difference {}",
            (self.scoreofeachsubject.science - another_student.science).abs()
        )
    }
}
