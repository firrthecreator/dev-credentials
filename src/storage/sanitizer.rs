pub struct Sanitizer;

impl Sanitizer {
    pub fn sanitize_filename(name: &str) -> String {
        let mut result = String::new();
        let mut last_was_underscore = false;

        for c in name.chars() {
            if c.is_ascii_alphanumeric() {
                result.push(c.to_ascii_lowercase());
                last_was_underscore = false;
            } else if !last_was_underscore && !result.is_empty() {
                result.push('_');
                last_was_underscore = true;
            }
        }

        while result.ends_with('_') {
            result.pop();
        }

        if result.is_empty() {
            "credential".to_string()
        } else {
            result
        }
    }

    pub fn sanitize_id(id: &str) -> String {
        id.replace(':', "").replace('/', "_")
    }
}
