use std::sync::OnceLock;
use tiktoken_rs::{CoreBPE, cl100k_base, model::get_context_size};

/// A thread-safe, lazily initialized tokenizer for GPT models
static TOKENIZER: OnceLock<CoreBPE> = OnceLock::new();


/// Gets the number of tokens in a string using GPT-3.5/4 tokenizer (cl100k_base)
pub fn count_tokens(text: &str, model: Option<&str>) -> (usize, usize) {
    let tokenizer = TOKENIZER.get_or_init(|| {
        cl100k_base().expect("Failed to load cl100k_base tokenizer")
    });

    let token_count = tokenizer.encode_with_special_tokens(text).len();
    let context_size = get_context_size(model.unwrap_or("gpt-3.5-turbo"));
    
    (token_count, context_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_count() {
        // Test basic token counting with different models
        let (count, context) = count_tokens("Hello", Some("gpt-4"));
        assert_eq!(count, 1);
        assert_eq!(context, 8192);
        
        let (count, context) = count_tokens("Hello, world!", Some("gpt-3.5-turbo"));
        assert_eq!(count, 4);
        assert_eq!(context, 16385);
        
        // Test empty string
        let (count, _) = count_tokens("", None);
        assert_eq!(count, 0);
        
        // Test whitespace
        let (count, _) = count_tokens("   ", None);
        assert_eq!(count, 1);
        
        // Test special characters
        let (count, _) = count_tokens("🦀", None);
        assert_eq!(count, 3);
        
        // Test longer text
        let long_text = "This is a longer piece of text that should be more than 20 tokens so we can test if the tokenizer is working correctly ...🤫";
        let (count, _) = count_tokens(long_text, None);
        assert!(count > 20);
    }
}
