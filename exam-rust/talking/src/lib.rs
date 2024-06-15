pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let aphabet = text.chars().any(|c| c.is_alphabetic());
    let yelling = text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_ascii_uppercase());
    let question = text.ends_with("?");

    if aphabet && yelling && !question {
        return "There is no need to yell, calm down!";
    } else if aphabet && yelling && question {
        return "Quiet, I am thinking!"
    } else if question {
        return "Sure."
    };
    "Interesting"
}

/* Other Solution
pub fn talking(input: &str) -> &str {
    if input.trim().is_empty() {
        "Just say something!"
    } else if input == input.to_uppercase() && input.chars().any(|c| c.is_alphabetic()) {
        if input.ends_with('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else if input.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}

 */