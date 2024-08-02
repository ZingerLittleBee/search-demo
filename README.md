# Search Demo

## 开发前准备
### 启动 surreal db 服务
```docker
docker-compose up -d
```

### 安装图片转提示词服务
1. 安装 [ollama](https://ollama.com/)
2. 安装 llava 模型，`ollama run llava`


### 启动 embed server
```shell
pip install -r embed-server/requirements.txt
cd embed-server && python -m uvicorn main:app --port 8080 --reload
```