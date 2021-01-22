#[cfg(test)]
mod test{
    use crate::parser::calc;
    #[test]
    fn passed_case(){
        let cases = vec![
            ("1 + 2", "3"),           
            ("3 - 1", "2"),           
            ("1111 + 3333", "4444"),           
            ("1 * 2", "2"),           
            ("1 / 2", "0.5"),           
            ("1", "1"),           
            ("(1 + 2) * 3", "9"),           
            ("1 + 2 * 3", "7"),           
            ("-1 * 2 - 3", "-5"),           
            //bad case ("12315742983075 + 8798273530745", "21114016513820"),
            //bad case ("0.0001 * 0.0001", "0.0000001"),
        ];
        for (arg, ret) in cases{
            assert_eq!(calc(arg), format!("ans = {}\n", ret));
        }
    }
}
