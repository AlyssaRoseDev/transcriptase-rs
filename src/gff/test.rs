use proptest::prelude::*;

use super::parsers::*;
use super::AttributeSet;

macro_rules! assume {
    ($e:expr) => {
        prop_assert!($e.is_ok())
    };
}

macro_rules! assume_err {
    ($e:expr) => {
        prop_assert!($e.is_err())
    };
}

macro_rules! strategy {
    () => {};
    ($fnid:ident: $ty:ty => $id:ident in $e:expr) => {
        prop_compose! {
            fn $fnid()($id in $e) -> $ty {
                $id
            }
        }
    };

    ($fnid:ident: $ty:ty => $id:ident in $e:expr, $($tail:tt)*) => {
        strategy!($fnid: $ty => $id in $e);
        strategy!{$($tail)*}
    };
}

strategy! {
    gen_seq_id: String => s in "[a-zA-Z0-9.:^*$@!+_?|%-]+",
    gen_source: String => s in any::<String>().prop_filter("Source must not contain reserved characters and must not be empty", |s| !s.contains(['\t', '\r', '\n']) && !s.is_empty()),
    gen_score: String => s in any::<Option<f64>>().prop_map(|r| if let Some(r) = r { r.to_string() } else { ".".to_string() }),
    gen_strand: String => s in "[.+?-]",
    gen_phase: String => p in "[.012]",
    gen_range: usize => r in any::<usize>().prop_filter("Ranges are 1 indexed and cannot be 0", |&r| (r != 0))
}

prop_compose! {
    fn gen_full_range(max: usize)(low in gen_range())(high in low..max, low in Just(low)) -> (usize, usize) {
       (low, high)
    }
}

prop_compose! {
   fn gen_attr()(key in r#"[a-z0-9.:^*$@!+_?| "'()/-]"#, val in r#"[a-zA-Z0-9.:^*$@!+_?| "'()/-]"#) -> String {
        format!("{key}={val}")
    }
}

prop_compose! {
    fn gen_attr_list()(vec in prop::collection::vec(gen_attr(), 1..25)) -> String {
        vec.join(";")
    }
}

prop_compose! {
    fn gen_entry()(seq in gen_seq_id(), source in gen_source(), feature in gen_source(), range in gen_full_range(usize::MAX), score in gen_score(), strand in gen_strand(), phase in gen_phase(), attrs in gen_attr_list()) -> String {
        let (low, high) = range;
        format!("{seq}\t{source}\t{feature}\t{low}\t{high}\t{score}\t{strand}\t{phase}\t{attrs}")
    }
}

proptest! {
    // ** Start Parser Tests **
    #[test]
    fn seq_ids(s in gen_seq_id()) {
        assume!(seq_id(&s))
    }

    #[test]
    fn sources(s in gen_source()) {
        assume!(source(&s))
    }

    #[test]
    fn feature_types(s in gen_source()) {
        assume!(feature_type(&s))
    }

    #[test]
    fn ranges(r in gen_range().prop_map(|r| r.to_string())) {
        assume!(range_bound(&r))
    }

    #[test]
    fn no_neg_range(r in (isize::MIN..-1).prop_map(|r| r.to_string())) {
        assume_err!(range_bound(&r))
    }

    #[test]
    fn scores(s in gen_score()) {
        assume!(score(&s))
    }

    #[test]
    fn strands(s in gen_strand()) {
        assume!(strand(&s))
    }

    #[test]
    fn phases(p in gen_phase()) {
        assume!(phase(&p))
    }

    #[test]
    fn attribute_list(a in gen_attr_list()) {
        assume!(attributes(&a));
        assume!(AttributeSet::parse(&a));
    }

    #[test]
    fn full_entry(test in gen_entry()) {
        assume!(entry(&test))
    }

    // ** End Parser Tests **
}

#[test]
fn no_uppercase_attr() {
    AttributeSet::parse("A=0").unwrap_err();
}
