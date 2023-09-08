use egui::{Ui, WidgetText};

pub fn label_wrap(ui: &mut Ui, x: impl Into<WidgetText>) {
    ui.add(egui::Label::new(x).wrap(true));
}

pub fn stripslashes(s: &str) -> Option<String> {
    let mut n = String::new();

    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        n.push(match c {
            '\\' => chars.next()?,
            c => c,
        });
    }

    Some(n)
}

pub fn remove_quotes(input: &str) -> String {
    let mut result = input.to_string();

    // Remove leading quotes
    if result.starts_with('"') {
        result.remove(0);
    }

    // Remove trailing quotes
    if result.ends_with('"') {
        result.pop();
    }

    // Replace escaped quotes with regular quotes
    result = result.replace("\\\"", "\"");

    result
}
