pub fn basename<'n>(name: &'n str, suffix: &str) -> &'n str {
    let mut target = name;

    if suffix.len() > 0 {
        target = name.strip_suffix(suffix).unwrap_or(name);
    }

    let result: Vec<&str> = target
        .strip_suffix("/")
        .unwrap_or(target)
        .split("/")
        .collect();

    result.last().unwrap()
}
