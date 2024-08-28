#[cfg(test)]
mod tests {

    use crate::process;

    #[test]
    fn basic_arithmetic() {
        assert_eq!(process("(1+1)*2/4-7".into(), false), -6.0);
    }

    #[test]
    fn other_arithmetic() {
        assert_eq!(process("2p2p2".into(), false), 16.0);

        assert_eq!(
            process(format!("sin{}", std::f32::consts::PI * 2.5).into(), false),
            1.0
        );
        assert_eq!(
            process(format!("cos{}", std::f32::consts::PI * 2.5).into(), false),
            0.0
        );
        assert_eq!(
            process(format!("tan{}", std::f32::consts::PI * 2.0).into(), false),
            0.0
        );
    }

    #[test]
    fn brackets() {
        assert_eq!(process("(1+1)".into(), false), 2.0);
        assert_eq!(process("(1+1)*2".into(), false), 4.0);
        assert_eq!(process("((1)+(1))".into(), false), 2.0);
        assert_eq!(process("(((1)))".into(), false), 1.0);
    }

    #[test]
    fn zeros() {
        assert_eq!(process("1+(-1)".into(), false), 0.0);
        assert_eq!(process("(-1)".into(), false), -1.0);
    }

    #[test]
    fn precise() {
        assert!(process("1/9".into(), false).to_string().len() < 5);
        assert!(process("1/9".into(), true).to_string().len() > 5);
    }
}
