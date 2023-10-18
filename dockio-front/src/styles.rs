use web_sys::Element;

pub fn visible_style() -> Element {
    let style = gloo::utils::document().create_element("style").unwrap();
    style.set_attribute("type", "text/css").unwrap();

    style.set_inner_html(&format!(r#".visible {{
        display: block;

        position: absolute;
        bottom: 20px;
        left: 20px;
        background-color: aqua;
        padding: 1rem;
        white-space: pre;
        font-family: monospace;
    }}"#));

    style
}

pub fn hidden_style() -> Element {
    let style = gloo::utils::document().create_element("style").unwrap();
    style.set_attribute("type", "text/css").unwrap();

    style.set_inner_html(&format!(r#".hidden {{
        display: none;
    }}"#));

    style
}
