use crate::*;

#[test]
fn dices() {
    let data = [
        ("d", DiroAst::Dice(Dice::d100(1, 0).unwrap(), None)),
        ("d100", DiroAst::Dice(Dice::d100(1, 0).unwrap(), None)),
        ("1d100", DiroAst::Dice(Dice::d100(1, 0).unwrap(), None)),
        ("1d100b", DiroAst::Dice(Dice::d100(1, 1).unwrap(), None)),
        ("1d100b1", DiroAst::Dice(Dice::d100(1, 1).unwrap(), None)),
        ("1d100p", DiroAst::Dice(Dice::d100(1, -1).unwrap(), None)),
        ("1d100p1", DiroAst::Dice(Dice::d100(1, -1).unwrap(), None)),
        ("2d100", DiroAst::Dice(Dice::d100(2, 0).unwrap(), None)),
        (
            "3d6",
            DiroAst::Dice(
                Dice::Dice {
                    count: 3,
                    face: 6,
                    kq: 0,
                },
                None,
            ),
        ),
        (
            "5a6",
            DiroAst::Dice(
                Dice::ADice {
                    count: 5,
                    face: 10,
                    add_line: 6,
                    success_line: 8,
                },
                None,
            ),
        ),
        (
            "3c6",
            DiroAst::Dice(
                Dice::CDice {
                    count: 3,
                    face: 10,
                    count_line: 6,
                },
                None,
            ),
        ),
        ("4f", DiroAst::Dice(Dice::FDice(4), None)),
    ];
    for (input, expected) in data.iter() {
        let mut result = parse(input).unwrap();
        assert_eq!(result, *expected);
        result.roll();
        println!(
            "{}={}={}",
            result.expr(),
            result.detail_expr().unwrap(),
            result.calc().unwrap()
        )
    }
}
