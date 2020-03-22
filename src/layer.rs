use std::borrow::Cow;

use conllu::token::{Features, Token};

pub type LayerCallback = Box<dyn Fn(&Token) -> Option<Cow<str>>>;

pub fn layer_callback(layer: &str) -> Option<LayerCallback> {
    match layer {
        "features" => Some(Box::new(|t| {
            t.features().map(Features::to_string).map(Cow::Owned)
        })),
        "form" => Some(Box::new(|t| Some(Cow::Borrowed(t.form())))),
        "lemma" => Some(Box::new(|t| t.lemma().map(Cow::Borrowed))),
        "upos" => Some(Box::new(|t| t.upos().map(Cow::Borrowed))),
        "xpos" => Some(Box::new(|t| t.xpos().map(Cow::Borrowed))),
        _ => None,
    }
}
