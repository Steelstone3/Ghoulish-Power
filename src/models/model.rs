pub struct Model {
    pub the_thing: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            the_thing: String::from("The thing"),
        }
    }
}

#[cfg(test)]
mod model_should {
    use super::*;

    #[test]
    fn have_the_thing() {
        //Arrange
        let model = Model::default();

        //Act

        //Assert
        assert_eq!(String::from("The thing"), model.the_thing);
    }
}
