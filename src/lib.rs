pub mod c_demo {
    include!(concat!(env!("OUT_DIR"), "/c_demo.rs"));

    #[cfg(test)]
    fn test(input: u64, output: u64) {
        assert_eq!(unsafe { f(input) }, output);
    }

    #[test]
    fn tests() {
        test(0, 0);
        test(1, 0);
        test(2, 1);
        test(3, 1);
        test(4, 2);
    }
}

pub mod cxx_demo {
    include!(concat!(env!("OUT_DIR"), "/cxx_demo.rs"));

    #[cfg(test)]
    fn test(input: u64, output: u64) {
        assert_eq!(unsafe { g(input) }, output);
    }

    #[test]
    fn tests() {
        test(0, 1);
        test(1, 2);
        test(2, 3);
        test(3, 4);
        test(4, 5);
    }
}
