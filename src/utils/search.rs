use rust_search::SearchBuilder;

pub(crate) fn paths(staring_path: &str) -> Vec<String> {
    return SearchBuilder::default()
        .location(staring_path)
        .search_input(".doctor-crab")
        .limit(1000)
        .ext("yaml")
        .depth(5)
        .ignore_case()
        .hidden()
        .build()
        .collect();
}
