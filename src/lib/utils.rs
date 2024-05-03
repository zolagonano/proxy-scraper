use regex::Regex;

/// Separate links from text and return them as a string.
pub fn seperate_links(text: &str) -> String {
    let regex = Regex::new(
        r#"\b(https|ss|vmess|vless|trojan|hysteria2|hy2|hysteria)?://[^\s<>"']+[^.,;!?)"'\s]"#,
    )
    .unwrap();
    let mut links = String::new();
    for cap in regex.captures_iter(text) {
        links.push_str(&cap[0].replace("&amp;amp;", "&").replace("%3D", "="));
        links.push('\n');
    }
    links
}
