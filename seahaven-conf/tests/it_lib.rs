use seahaven_conf::add;

#[test]
fn add_two_numbers() {
    //* Given
    let left = 2;
    let right = 2;

    //* When
    let result = add(left, right);

    //* Then
    assert_eq!(result, 4);
}
