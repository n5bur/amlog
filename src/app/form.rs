// src/app/form.rs
pub struct FormField {
    pub label: String,
    pub value: String,
    pub required: bool,
    pub cursor_position: usize,
}

pub struct Form {
    pub fields: Vec<FormField>,
    pub current_field: usize,
}

impl Form {
    pub fn new() -> Self {
        let fields = vec![
            FormField {
                label: "Callsign".to_string(),
                value: String::new(),
                required: true,
                cursor_position: 0,
            },
            FormField {
                label: "Frequency".to_string(),
                value: String::new(),
                required: true,
                cursor_position: 0,
            },
            FormField {
                label: "Mode".to_string(),
                value: String::new(),
                required: true,
                cursor_position: 0,
            },
            FormField {
                label: "RST Sent".to_string(),
                value: String::new(),
                required: false,
                cursor_position: 0,
            },
            FormField {
                label: "RST Rcvd".to_string(),
                value: String::new(),
                required: false,
                cursor_position: 0,
            },
            FormField {
                label: "Notes".to_string(),
                value: String::new(),
                required: false,
                cursor_position: 0,
            },
        ];
        Form {
            fields,
            current_field: 0,
        }
    }

    pub fn reset(&mut self) {
        for field in &mut self.fields {
            field.value.clear();
            field.cursor_position = 0;
        }
        self.current_field = 0;
    }

    pub fn next_field(&mut self) {
        self.current_field = (self.current_field + 1) % self.fields.len();
    }

    pub fn previous_field(&mut self) {
        if self.current_field == 0 {
            self.current_field = self.fields.len() - 1;
        } else {
            self.current_field -= 1;
        }
    }

    pub fn input(&mut self, c: char) {
        let field = &mut self.fields[self.current_field];
        field.value.insert(field.cursor_position, c);
        field.cursor_position += 1;
    }

    pub fn backspace(&mut self) {
        let field = &mut self.fields[self.current_field];
        if field.cursor_position > 0 {
            field.cursor_position -= 1;
            field.value.remove(field.cursor_position);
        }
    }

    pub fn is_valid(&self) -> bool {
        self.fields
            .iter()
            .filter(|field| field.required)
            .all(|field| !field.value.trim().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_validation() {
        let mut form = Form::new();
        assert!(!form.is_valid());

        // Fill required fields
        form.fields[0].value = "W1AW".to_string();
        form.fields[1].value = "14.074".to_string();
        form.fields[2].value = "FT8".to_string();

        assert!(form.is_valid());
    }
}