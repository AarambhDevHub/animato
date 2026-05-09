//! DOM text splitting helpers.

use wasm_bindgen::JsValue;
use web_sys::Element;

/// Text splitting mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplitMode {
    /// Split into Unicode scalar-value characters.
    Chars,
    /// Split on whitespace into words.
    Words,
}

/// Result of splitting an element's text into child spans.
#[derive(Clone, Debug)]
pub struct SplitText {
    element: Element,
    original_text: String,
    spans: Vec<Element>,
    mode: SplitMode,
}

impl SplitText {
    /// Split an element's text into character spans.
    pub fn chars(element: &Element) -> Result<Self, JsValue> {
        Self::split(element, SplitMode::Chars)
    }

    /// Split an element's text into word spans.
    pub fn words(element: &Element) -> Result<Self, JsValue> {
        Self::split(element, SplitMode::Words)
    }

    /// Split an element's text with a specific mode.
    pub fn split(element: &Element, mode: SplitMode) -> Result<Self, JsValue> {
        let document = element
            .owner_document()
            .ok_or_else(|| JsValue::from_str("element has no owner document"))?;
        let original_text = element.text_content().unwrap_or_default();
        element.set_text_content(None);

        let parts: Vec<String> = match mode {
            SplitMode::Chars => original_text.chars().map(|c| c.to_string()).collect(),
            SplitMode::Words => original_text
                .split_whitespace()
                .map(ToOwned::to_owned)
                .collect(),
        };

        let mut spans = Vec::with_capacity(parts.len());
        for (index, part) in parts.iter().enumerate() {
            if matches!(mode, SplitMode::Words) && index > 0 {
                element.append_child(&document.create_text_node(" "))?;
            }

            let span = document.create_element("span")?;
            span.set_text_content(Some(part));
            span.set_attribute("data-animato-split", "true")?;
            span.set_attribute("data-animato-index", &index.to_string())?;
            element.append_child(&span)?;
            spans.push(span);
        }

        Ok(Self {
            element: element.clone(),
            original_text,
            spans,
            mode,
        })
    }

    /// Borrow the generated spans.
    pub fn spans(&self) -> &[Element] {
        &self.spans
    }

    /// Return the splitting mode.
    pub fn mode(&self) -> SplitMode {
        self.mode
    }

    /// Restore the original text content.
    pub fn restore(&self) {
        self.element.set_text_content(Some(&self.original_text));
    }
}
