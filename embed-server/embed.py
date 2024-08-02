import base64
import io

from sentence_transformers import SentenceTransformer
from PIL import Image
import requests

# 初始化模型
img_model = SentenceTransformer('clip-ViT-B-32')
text_model = SentenceTransformer('sentence-transformers/clip-ViT-B-32-multilingual-v1')


def load_image(url_or_path):
    if url_or_path.startswith("http://") or url_or_path.startswith("https://"):
        return Image.open(requests.get(url_or_path, stream=True).raw)
    else:
        return Image.open(url_or_path)


def base64_to_image(base64_string):
    """
    将base64字符串转换为PIL Image对象
    :param base64_string: base64编码的图像数据
    :return: PIL Image对象
    """
    img_data = base64.b64decode(base64_string)
    return Image.open(io.BytesIO(img_data))


def image_to_vector(base64_images):
    """
    将图像转换为向量
    :param image_paths: 图像路径或URL列表
    :return: 图像向量
    """

    images = [base64_to_image(img) for img in base64_images]
    return img_model.encode(images)


def text_to_vector(texts):
    """
    将文本转换为向量
    :param texts: 文本列表
    :return: 文本向量
    """
    return text_model.encode(texts)


def image_to_base64(image):
    buffered = io.BytesIO()
    image.save(buffered, format="PNG")
    img_str = base64.b64encode(buffered.getvalue())
    return img_str.decode('utf-8')


# 示例使用
if __name__ == "__main__":
    # 图像到向量示例
    base64_image = image_to_base64(load_image("../test/image.png"))
    print("base64:", base64_image)
    img_vectors = image_to_vector([base64_image])
    print("Image vectors:", img_vectors)

    # 文本到向量示例
    texts = [
        "A dog in the snow",
        "Eine Katze",
        "Una playa con palmeras."
    ]
    text_vectors = text_to_vector(texts)
    print("Text vectors:", text_vectors)
