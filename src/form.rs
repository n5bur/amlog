use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;
use amlog::LogEntry;


#[derive(Default)]
pub struct FormField {
    pub label: String,
    pub value: String,
    pub cursor_position: usize,
}

impl FormField {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            value: String::new(),
            cursor_position: 0,
        }
    }

    pub fn with_value(label: &str, value: String) -> Self {
        let cursor_position = value.len();
        Self {
            label: label.to_string(),
            value,
            cursor_position,
        }
    }
}

pub struct LogForm {
    pub fields: Vec<FormField>,
    pub current_field: usize,
}

impl LogForm {
    pub fn new() -> Self {
        let utc_now = Utc::now();
        let fields = vec![
            FormField::new("Callsign"),
            FormField::with_value("Time UTC", utc_now.format("%H:%M").to_string()),
            FormField::with_value("Date UTC", utc_now.format("%Y-%m-%d").to_string()),
            FormField::new("RST Sent"),
            FormField::new("RST Rcvd"),
            FormField::new("Operator"),
            FormField::new("QTH"),
            FormField::new("State"),
            FormField::new("County"),
            FormField::new("Grid"),
            FormField::new("Freq"),
            FormField::new("Band"),
            FormField::new("Power"),
            FormField::new("Mode"),
            FormField::new("My Callsign"),
            FormField::new("My Grid"),
            FormField::new("Notes"),
        ];
        Self {
            fields,
            current_field: 0,
        }
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

    pub fn handle_input(&mut self, c: char) {
        let field = &mut self.fields[self.current_field];
        field.value.insert(field.cursor_position, c);
        field.cursor_position += 1;
    }

    pub fn handle_backspace(&mut self) {
        let field = &mut self.fields[self.current_field];
        if field.cursor_position > 0 {
            field.value.remove(field.cursor_position - 1);
            field.cursor_position -= 1;
        }
    }

    pub fn handle_delete(&mut self) {
        let field = &mut self.fields[self.current_field];
        if field.cursor_position < field.value.len() {
            field.value.remove(field.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        let field = &mut self.fields[self.current_field];
        if field.cursor_position > 0 {
            field.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let field = &mut self.fields[self.current_field];
        if field.cursor_position < field.value.len() {
            field.cursor_position += 1;
        }
    }
    pub fn to_log_entry(&self) -> Result<LogEntry, String> {
        // Helper function to get field value by label
        let get_field = |label: &str| -> Option<String> {
            self.fields
                .iter()
                .find(|f| f.label == label)
                .map(|f| f.value.clone())
                .filter(|v| !v.is_empty())
        };

        // Required fields
        let callsign = get_field("Callsign")
            .ok_or_else(|| "Callsign is required".to_string())?;

        // Parse frequency
        let frequency = get_field("Freq")
            .ok_or_else(|| "Frequency is required".to_string())?
            .parse::<f64>()
            .map_err(|_| "Invalid frequency format".to_string())?;

        // Parse power if provided
        let power = get_field("Power")
            .map(|p| p.parse::<f32>())
            .transpose()
            .map_err(|_| "Invalid power format".to_string())?;

        // Parse timestamp from date and time fields
        let date_str = get_field("Date UTC")
            .ok_or_else(|| "Date is required".to_string())?;
        let time_str = get_field("Time UTC")
            .ok_or_else(|| "Time is required".to_string())?;
        
        let timestamp = NaiveDateTime::parse_from_str(
            &format!("{} {}", date_str, time_str),
            "%Y-%m-%d %H:%M"
        )
        .map_err(|_| "Invalid date/time format".to_string())?;
        
        let timestamp = DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc);

        Ok(LogEntry {
            id: Uuid::new_v4().to_string(),
            timestamp,
            callsign,
            frequency,
            mode: get_field("Mode").unwrap_or_else(|| "".to_string()),
            rst_sent: get_field("RST Sent"),
            rst_received: get_field("RST Rcvd"),
            operator: get_field("Operator"),
            qth: get_field("QTH"),
            state: get_field("State"),
            county: get_field("County"),
            grid: get_field("Grid"),
            band: get_field("Band"),
            power,
            notes: get_field("Notes"),
            my_callsign: get_field("My Callsign"),
            my_grid: get_field("My Grid"),
            custom_fields: Default::default(),
        })
    }
}