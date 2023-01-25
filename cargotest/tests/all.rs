use cargotestlib::add;

#[test]
fn test_add() {
    let x = add(10, 20);
    assert_eq!(x, 30);
}
