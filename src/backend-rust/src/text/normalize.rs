use crate::types::{Term};

// returns true if c in ({A..Z} U {a..z} U {0..9})
fn is_alphanumeric(b: char) -> bool {
    ('a' <= b && b <= 'z') 
    || ('A' <= b && b <= 'Z') 
    || ('0' <= b && b <= '9')
}

// replace all punctuation by whitespace so it can be more easily split
pub fn replace_punctuation_whitespace<T>(token: T) -> Term
where T: Into<String>
{
    let s : String = token.into();

    // match all non-letter/number characters
    s.replace(|b: char| !is_alphanumeric(b), " ")
}

// convert all chars to lowercase
pub fn case_fold<T>(token: T) -> Term
where T: Into<String>
{
    let s : String = token.into();
    s.to_ascii_lowercase()
}

// porter stemmer
pub fn stemmer<T>(token: T) -> Term
where T: Into<String>
{
    token.into()
}

// stop word: in stop words list OR len <= 1
pub fn is_stop_word<T>(token: T) -> bool
where T: Into<String>
{
    // stop words list: https://gist.github.com/sebleier/554280
    const STOP_WORDS: &'static [&'static str] = &["i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her", "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "should", "now"];

    let s : String = token.into();
    s.len() <= 1 || (*STOP_WORDS).contains(&(s.as_str()))
}
