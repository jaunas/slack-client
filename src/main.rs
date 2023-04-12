use gtk::{prelude::*, Window, WindowType};
use webkit2gtk::{
    CookieManagerExt, CookiePersistentStorage, NavigationPolicyDecision,
    NavigationPolicyDecisionExt, URIRequestExt, WebContext, WebView, WebViewExt,
    WebsiteDataManagerExt,
};

const SLACK_URL: &str = "https://app.slack.com";
const SLACK_CLIENT_URL: &str = "https://app.slack.com/client";

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_default_size(1100, 900);
    window.set_title("Slack");
    window.set_icon_name(Some("gtk-network"));

    let context = WebContext::default().unwrap();
    let webview = WebView::with_context(&context);
    setup_cookies(&webview);
    setup_links(&webview);

    webview.load_uri(SLACK_CLIENT_URL);
    window.add(&webview);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

fn setup_cookies(webview: &WebView) {
    let mut app_data_dir = dirs::data_local_dir().unwrap();
    app_data_dir.push("slack-client");
    let mut cookie_path = app_data_dir.clone();
    cookie_path.push("cookies");

    if !std::path::Path::new(&app_data_dir).exists() {
        std::fs::create_dir(app_data_dir).unwrap();
    }

    webview
        .website_data_manager()
        .unwrap()
        .cookie_manager()
        .unwrap()
        .set_persistent_storage(cookie_path.to_str().unwrap(), CookiePersistentStorage::Text);
}

fn setup_links(webview: &WebView) {
    webview.connect_decide_policy(|_, policy_decision, _| {
        if let Some(policy) = policy_decision.dynamic_cast_ref::<NavigationPolicyDecision>() {
            if let Some(nav_action) = policy.navigation_action() {
                if let Some(uri_req) = nav_action.request() {
                    if let Some(uri) = uri_req.uri() {
                        let uri = uri.to_string();
                        if !uri.starts_with(SLACK_URL) && !uri.starts_with("about:") {
                            open::that(uri).unwrap();
                        }
                    }
                }
            }
        }
        true
    });
}
