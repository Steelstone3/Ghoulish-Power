#[allow(dead_code)]
pub fn do_the_thing() -> bool {
    return true;
}

#[cfg(test)]
mod controller_should {
    use super::*;

    #[test]
    fn do_the_thing_successfully() {
        //Arrange

        //Act
        let result = do_the_thing();

        //Assert
        assert_eq!(true, result);
    }
}
