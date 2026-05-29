use pulldown_cmark::{Options, Parser};
use std::env;
use std::fs;

pub fn process_md(md_input: &str, skip_template: bool) -> String {
    let md_html: String = parse_md(md_input);
    if skip_template {
        let parsed_template = inject_styles(&md_html);
        return parsed_template;
    } else {
        let template: String = load_template();
        let parsed_template = template.replace("%%PAGE_CONTENT(html/index.html)%%", &md_html);
        return inject_styles(&parsed_template);
    }
}

fn inject_styles(content: &str) -> String {
    let mut content_with_styles: String = String::from(content);
    content_with_styles = content_with_styles.replace("<p", "<p class=\"text-base blog-block\"");
    content_with_styles = content_with_styles.replace("<h2", "<h2 class=\"text-4xl\"");
    content_with_styles = content_with_styles.replace("<h3", "<h3 class=\"text-2xl\"");
    content_with_styles = content_with_styles.replace("<h4", "<h4 class=\"text-2xl\"");
    content_with_styles = content_with_styles.replace("<a", "<a class=\"text-green-800\"");
    return content_with_styles;
}

fn load_template() -> String {
    let site_path: String = env::var("SITE_PATH").unwrap_or_else(|_| "/tmp/website".to_string());
    let contents = fs::read_to_string(site_path + "/index.html")
        .expect("Should have been able to read the file");
    return contents;
}

fn parse_md(md_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md_input, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    return html_output;
}
