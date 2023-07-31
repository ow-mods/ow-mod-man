use unicode_normalization::UnicodeNormalization;

/// Represents an object that can be searched
pub trait Searchable {
    /// Get the values that can be searched
    /// Each value will be weighted based on its position in the list (first is most important)
    fn get_values(&self) -> Vec<String>;
}

// - Lowercase the value
// - Perform unicode normalization
// - Remove any spaces or whitespace or control characters
// - Remove any accents on characters
// \u{0300} is the start of the combining diacritical marks unicode block, \u{036f} is the end
fn normalize_value(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .chars()
        .nfd()
        .filter(|c| {
            !c.is_whitespace() && !c.is_control() && !((&'\u{0300}'..=&'\u{036f}').contains(&c))
        })
        .collect()
}

/// Search a list of [Searchable] for a string
/// This will return a list of the items that match the search, sorted by relevance
/// Relevance is determined like so:
/// - If the search is an exact match for a value, that value will be weighted 2x
/// - If the search is contained in a value, that value will be weighted 1x
/// - If the search is not contained in a value, that value will be weighted 0x
/// The score is then also weighted by the position of the value in the list the [Searchable] (first is most important)
/// These scores are then summed and the list is sorted by the total score of each item
pub fn search_list<'a, T>(source_list: Vec<&'a T>, filter: &str) -> Vec<&'a T>
where
    T: Searchable,
{
    let filter = dbg!(normalize_value(filter));
    let mut scores: Vec<(&T, f32)> = source_list
        .into_iter()
        .filter_map(|m| {
            let final_score = m
                .get_values()
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let weight = 1.0 - (index as f32 / 10.0 * 2.0);
                    let field = normalize_value(field);
                    let mut score = if field == filter {
                        2.0
                    } else if field.contains(&filter) {
                        1.0
                    } else {
                        0.0
                    };
                    score *= weight;
                    score
                })
                .sum();
            if final_score != 0.0 {
                Some((m, final_score))
            } else {
                None
            }
        })
        .collect();
    scores.sort_by(|(_, a), (_, b)| a.total_cmp(b).reverse());
    scores.iter().map(|(m, _)| *m).collect()
}

#[cfg(test)]
pub mod tests {

    use super::*;

    struct TestStruct {
        name: String,
        description: String,
    }

    impl Searchable for TestStruct {
        fn get_values(&self) -> Vec<String> {
            vec![self.name.clone(), self.description.clone()]
        }
    }

    fn make_test_structs() -> Vec<TestStruct> {
        vec![
            TestStruct {
                name: "Test".to_string(),
                description: "This is a test".to_string(),
            },
            TestStruct {
                name: "Test2".to_string(),
                description: "This is a test2".to_string(),
            },
            TestStruct {
                name: "Test3".to_string(),
                description: "This is a test3".to_string(),
            },
        ]
    }

    #[test]
    fn test_search() {
        let test_structs = make_test_structs();
        let results = search_list(test_structs.iter().collect(), "test");
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].name, "Test".to_string());
        assert_eq!(results[1].name, "Test2".to_string());
        assert_eq!(results[2].name, "Test3".to_string());
    }

    #[test]
    fn test_search_exact() {
        let test_structs = make_test_structs();
        let results = search_list(test_structs.iter().collect(), "test2");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test2".to_string());
    }

    #[test]
    fn test_search_case_insensitive() {
        let test_structs = make_test_structs();
        let results = search_list(test_structs.iter().collect(), "TEST2");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test2".to_string());
    }

    #[test]
    fn test_search_with_spaces() {
        let test_structs = make_test_structs();
        let results = search_list(test_structs.iter().collect(), "test 2");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test2".to_string());
    }

    #[test]
    fn test_search_with_control_characters() {
        let test_structs = make_test_structs();
        let results = search_list(test_structs.iter().collect(), "test\u{0002}2");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test2".to_string());
    }

    #[test]
    fn test_search_with_accents() {
        let test_structs = make_test_structs();
        let results = search_list(test_structs.iter().collect(), "tëst2");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test2".to_string());
    }
}
