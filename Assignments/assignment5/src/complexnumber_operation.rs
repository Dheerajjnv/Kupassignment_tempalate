//Creating Student of structure type
//
//#Argument
//real and imaginary part
//
//Return after performing addition,subtraction,multiplication

#[derive(Clone, Copy)]
pub(crate) struct ComplexNumber {
    pub(crate) real_part: i32,
    pub(crate) imaginary_part: i32,
}
//Implementation of structure method
impl ComplexNumber {
    //Addition performed
    pub(crate) fn addition(&self, second_no: ComplexNumber) -> String {
        let result_real: i32 = self.real_part + second_no.real_part;
        let result_imaginary: i32 = self.imaginary_part + second_no.imaginary_part;
        format!("{}+i{}", result_real, result_imaginary)
    }
    //Subtraction performed
    pub fn subtraction(&self, second_no: ComplexNumber) -> String {
        let result_real: i32 = self.real_part - second_no.real_part;
        let result_imaginary: i32 = self.imaginary_part - second_no.imaginary_part;
        format!("{}+i{}", result_real, result_imaginary)
    }
    //Multiplication performed
    pub(crate) fn multiplication(&self, second_no: ComplexNumber) -> String {
        let result_real: i32 = (self.real_part * second_no.real_part)
            - (self.imaginary_part * second_no.imaginary_part);
        let result_imaginary: i32 = (self.real_part * second_no.imaginary_part)
            + (self.imaginary_part * second_no.real_part);
        format!("{}+i{}", result_real, result_imaginary)
    }
}
