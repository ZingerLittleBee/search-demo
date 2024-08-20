import json

from minio import Minio
from datetime import timedelta

# 初始化MinIO客户端
client = Minio(
    "localhost:9000",
    access_key="ROOTNAME",
    secret_key="CHANGEME123",
    secure=False
)

# 指定桶名和路径前缀
bucket_name = "muse"
prefix = "image/"  # 确保以斜杠结尾

# 列出目录下的所有文件
objects = client.list_objects(bucket_name, prefix=prefix, recursive=False)

# 获取文件名列表
file_names = [obj.object_name for obj in objects]

# 生成每个文件的下载URL
download_urls = []
for file_name in file_names:
    url = client.presigned_get_object(bucket_name, file_name, expires=timedelta(hours=1))
    download_urls.append(url)

# 打印下载URL
for url in download_urls:
    image = {
        "url": url
    }
    print('''curl -X POST -H "Content-Type: application/json" -d '{}' http://localhost:3000/inbound/image'''.format(json.dumps(image)))