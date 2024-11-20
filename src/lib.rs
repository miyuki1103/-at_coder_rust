mod daily_training;

#[cfg(test)]
mod q_2024_11_19_1530 {
    use super::*;

    #[test]
    fn a1() {
        let result= daily_training::ez_a(Vec::from([4,3,3,6,2,5,10]));
        assert_eq!(result, "Takahashi".to_string());
    }
    #[test]
    fn a2() {
        let result= daily_training::ez_a(Vec::from([3,1,4,1,5,9,2]));
        assert_eq!(result, "Aoki".to_string());
    }
    #[test]
    fn a3() {
        let result= daily_training::ez_a(Vec::from([1,1,1,1,1,1,1]));
        assert_eq!(result, "Draw".to_string());
    }
}