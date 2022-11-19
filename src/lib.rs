use digenum::DigEnum;

#[derive(DigEnum, PartialEq, Eq)]
pub enum PubEnum<A, B, C> {
    Empty,
    Single(u32),
    Double(char, char),
    Generic(A, B, C),
}

#[cfg(test)]
type TestEnum = PubEnum<(), u8, char>;

#[test]
pub fn into() {
    assert_eq!(0, TestEnum::Single(0).into_Single().unwrap());
    assert_eq!(
        ('x', 'x'),
        TestEnum::Double('x', 'x').into_Double().unwrap()
    );
    assert_eq!(
        ((), 1, 'a'),
        TestEnum::Generic((), 1, 'a').into_Generic().unwrap()
    );
}

#[test]
pub fn as_ref() {
    type TestEnum = PubEnum<(), u8, char>;
    assert_eq!(&0, TestEnum::Single(0).as_Single().unwrap());
    assert_eq!(
        (&'x', &'x'),
        TestEnum::Double('x', 'x').as_Double().unwrap()
    );
    assert_eq!(
        (&(), &1, &'a'),
        TestEnum::Generic((), 1, 'a').as_Generic().unwrap()
    );
}

#[test]
pub fn as_mut() {
    assert_eq!(&mut 0, TestEnum::Single(0).as_mut_Single().unwrap());
    assert_eq!(
        (&mut 'x', &mut 'x'),
        TestEnum::Double('x', 'x').as_mut_Double().unwrap()
    );
    assert_eq!(
        (&mut (), &mut 1, &mut 'a'),
        TestEnum::Generic((), 1, 'a').as_mut_Generic().unwrap()
    );
}

#[test]
pub fn none() {
    assert_eq!(None, TestEnum::Single(0).into_Double());
    assert_eq!(None, TestEnum::Double('x', 'x').as_Generic());
    assert_eq!(None, TestEnum::Generic((), 1, 'a').as_mut_Single());
}
