use url::Url;

pub struct TextInputData(pub String);

pub struct ImageInputData {
    pub url: Url,
    pub data: Vec<u8>,
}

pub struct ItemInputData {
    text: Vec<TextInputData>,
    image: Vec<ImageInputData>,
}

pub enum InputData {
    Text(TextInputData),
    Image(ImageInputData),
    Item(ItemInputData),
}
