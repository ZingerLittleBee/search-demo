use crate::model::image::ImageModel;
use crate::model::text::TextModel;

pub enum InputData {
    Image(ImageModel),
    Text(TextModel),
    Item(Vec<TextModel>, Vec<ImageModel>),
}
