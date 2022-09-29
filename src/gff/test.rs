use proptest::prelude::*;

use super::parsers::*;

macro_rules! assume {
    ($e:expr) => {
        prop_assert!($e.is_ok())
    };
}

prop_compose! {
    fn valid_ascii()(s in "[a-zA-Z0-9.:^*$@!+_?|%-]") -> String {
        s
    }
}

proptest! {
    #[test]
    fn seq_id(s in valid_ascii()) {
        prop_assert!(super::parsers::seq_id(&s).is_ok())
    }

    //this should test for any valid unicode which is not a reserved character
    //however, I am sleepy and cannot figure out how to define such a strategy
    //TODO make this right
    #[test]
    fn sources(s in valid_ascii()) {
        assume!(source(&s))
    }

    #[test]
    fn feature_types(s in valid_ascii()) {
        assume!(feature_type(&s))
    }

    #[test]
    fn ranges(r in any::<usize>().prop_map(|r| r.to_string())) {
        assume!(range_bound(&r))
    }

    #[test]
    fn scores(s in any::<f64>().prop_map(|s| s.to_string())) {
        assume!(score(&s))
    }

    #[test]
    fn strands(s in "[.+?-]") {
        assume!(strand(&s))
    }

    #[test]
    fn phases(p in "[.012]") {
        assume!(phase(&p))
    }
}
