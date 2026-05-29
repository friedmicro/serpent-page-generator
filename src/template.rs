use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub struct Replacements {
    inner: HashMap<String, String>,
}

impl Default for Replacements {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl Replacements {
    pub fn set(&mut self, key: &str, value: String) {
        self.inner.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).cloned()
    }
}

pub fn render(template: &str, replacements: &Replacements) -> String {
    let mut templated_string: Vec<String> = Vec::new();
    for line in template.lines() {
        if line.contains("%%PAGE_CONTENT(") {
            let rough_templated_line = template_file_inject(line);
            let templated_line = render(rough_templated_line.as_str(), replacements);
            templated_string.push(templated_line);
        } else if line.contains("%%") {
            let templated_line = inject_variable(line, replacements);
            templated_string.push(templated_line);
        } else {
            templated_string.push(String::from(line));
        }
    }
    return templated_string.join("\n");
}

fn inject_variable(line: &str, replacements: &Replacements) -> String {
    let (variable_name, variable_prefix) = get_variable_name(line);
    let value = replacements.get(variable_name);
    let templated_line = match value {
        Some(v) => line.replace(&variable_prefix, &v),
        None => line.to_string(),
    };
    return templated_line;
}

fn template_file_inject(template_line: &str) -> String {
    let site_path: String = env::var("SITE_PATH").unwrap_or_else(|_| "/tmp/website".to_string());
    let line_parts: Vec<&str> = template_line.split("%%PAGE_CONTENT(").collect();
    let template_path_parts: Vec<&str> = line_parts[1].split(")").collect(); //.split(")")[0];
    let relative_template_path = template_path_parts[0];
    let full_template_path: String;
    if template_line.contains("%%PAGE_CONTENT(out/") {
        full_template_path = String::from(relative_template_path);
    } else {
        full_template_path = site_path + "/" + relative_template_path;
    }
    let variable_string = "%%PAGE_CONTENT(".to_string() + relative_template_path + ")%%";
    let contents = fs::read_to_string(Path::new(full_template_path.as_str()))
        .expect("Should have been able to read the file");
    let templated_line = template_line.replace(variable_string.as_str(), contents.as_str());
    return String::from(templated_line);
}

fn get_variable_name(line: &str) -> (&str, String) {
    let variable_delimiter: String = String::from("%%");
    let parts: Vec<&str> = line.split("%%").collect();
    let variable_name: &str = parts[1];
    let mut variable_full_prefixes: String = variable_delimiter + variable_name;
    variable_full_prefixes += "%%";
    return (variable_name, variable_full_prefixes);
}
