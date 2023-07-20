pub trait Searchable {
    fn get_values(&self) -> Vec<String>;
}

pub fn search_list<'a, T>(source_list: Vec<&'a T>, filter: &str) -> Vec<&'a T>
where
    T: Searchable,
{
    let filter = filter.to_ascii_lowercase();
    let mut scores: Vec<(&T, f32)> = source_list
        .into_iter()
        .filter_map(|m| {
            let final_score = m
                .get_values()
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let weight = 1.0 - (index as f32 / 10.0 * 2.0);
                    let mut score = if field == &filter {
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
}
