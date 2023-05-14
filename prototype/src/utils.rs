pub fn path_join(segments: Vec<&str>) -> String {
    segments.join(std::path::MAIN_SEPARATOR_STR)
}

pub fn sprite(s: &str) -> String {
    path_join(vec!["sprites", s])
}
