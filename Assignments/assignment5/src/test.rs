#[cfg(test)]
use crate::complexnumber_operation::ComplexNumber;
use crate::studentexam_operation::{Score, Student};

#[test]
//testing for complex no. operation
fn test() {
    let number = ComplexNumber {
        real_part: 4,
        imaginary_part: 3,
    };
    let second_no = ComplexNumber {
        real_part: 5,
        imaginary_part: 6,
    };
    let addition_output = second_no.addition(number);
    let subtraction_output = second_no.subtraction(number);
    let multiplication_output = second_no.multiplication(number);
    assert_eq!(addition_output, "9+i9");
    assert_eq!(subtraction_output, "1+i3");
    assert_eq!(multiplication_output, "2+i39");
}
//testing for student score operation
#[test]
fn student_information_test() {
    let first_student = Student::new(&Score {
        hindi: 52,
        english: 20,
        maths: 92,
        science: 36,
    });
    let output = first_student.average_calculator();
    assert_eq!(output, 50.0);
}

#[test]
fn pass_student() {
    let student_marks1 = Score {
        hindi: 30,
        english: 32,
        maths: 34,
        science: 65,
    };
    let mut check_test = Student::new(&student_marks1);
    check_test.guess_mark();
    assert_eq!(check_test.scoreofeachsubject.hindi, 35);
    assert_eq!(check_test.scoreofeachsubject.english, 32);
    assert_eq!(check_test.scoreofeachsubject.maths, 34);
    assert_eq!(check_test.scoreofeachsubject.science, 65);
}
