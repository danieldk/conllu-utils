use std::borrow::Cow;

use conllu::display::{ConlluFeatures, ConlluMisc};
use udgraph::token::Token;

pub type LayerCallback = Box<dyn Fn(&Token) -> Option<Cow<str>>>;

pub fn layer_callback(layer: &str) -> Option<LayerCallback> {
    match layer {
        "features" => Some(Box::new(|t| {
            Some(Cow::Owned(
                ConlluFeatures::borrowed(t.features()).to_string(),
            ))
        })),
        "form" => Some(Box::new(|t| Some(Cow::Borrowed(t.form())))),
        "lemma" => Some(Box::new(|t| t.lemma().map(Cow::Borrowed))),
        "misc" => Some(Box::new(|t| {
            Some(Cow::Owned(ConlluMisc::borrowed(t.misc()).to_string()))
        })),
        "upos" => Some(Box::new(|t| t.upos().map(Cow::Borrowed))),
        "xpos" => Some(Box::new(|t| t.xpos().map(Cow::Borrowed))),
        _ => None,
    }
}
