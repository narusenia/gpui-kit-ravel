mod showcase;

fn main() {
    let component = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "overview".to_string());

    // gpui-ce ships no reqwest-backed `HttpClient`, and nothing in the showcase
    // fetches a remote resource, so the app keeps gpui's default
    // `NullHttpClient` rather than being handed one here.
    showcase::run(gpui_platform::application(), component);
}
