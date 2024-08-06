use crate::model::image::ImageModel;
use crate::model::text::TextModel;

pub struct ItemModel {
    pub text: Vec<TextModel>,
    pub image: Vec<ImageModel>,
}
