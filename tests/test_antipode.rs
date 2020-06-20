use antipode::antipode;

macro_rules! antipode_tests {
    ($($name:ident: $value:tt),*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, antipode(input));
            }
        )*
    }
}

antipode_tests! {
    test_1: ((27.97,     -73.951442),  (-27.97,      106.048558)),
    test_2: ((48.388286, -115.555999), (-48.388286,  64.444001)),
    test_3: ((60.394306,  5.325919),   (-60.394306, -174.674081)),
    test_4: ((-33.854816, 151.216454), (33.854816,  -28.783539))
}
