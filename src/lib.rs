mod DailyTraining;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn something() {
        let result= DailyTraining::ez_a();
        assert_eq!(result, "ez".to_string());
    }
}