impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_chars = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
        
        s_chars.clone().eq(s_chars.rev())
    }
}
