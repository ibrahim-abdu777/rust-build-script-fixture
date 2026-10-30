pub fn normalize_channel(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(' ', "-")
}

#[cfg(test)]
mod tests {
    use super::normalize_channel;

    #[test]
    fn normalizes_release_channel() {
        assert_eq!(normalize_channel("Release Candidate"), "release-candidate");
    }
}
