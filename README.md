# Search Demo

## 如何使用
```bash
docker-compose up -d
```
访问 [http://localhost:3000](http://localhost:3000)


## 如何开发
### 启动 surreal db 服务
```docker
docker-compose up -d
```

### 创建环境变量
```shell
cp .env.example .env
```

### 安装图片转提示词服务
1. 安装 [ollama](https://ollama.com/)
2. 安装 llava 模型，`ollama run llava`


### 下载模型
```shell
bash scripts/download-models.sh
```

## SurrealDB 使用示例
```sql
-- 创建一个 id 为 'text:aaa' 的 text 记录
-- 并将返回值（id）存储到 $text 变量中
LET $text = (CREATE text:aaa CONTENT {
	data: 'vaaa',
	vector: [0.1,0.1,0.1]
}).id;

-- 创建一个 id 为 'text:aaa' 的 image 记录
-- 并将返回值 (id) 存储到 $image 变量中
LET $image = (CREATE image:bbb CONTENT {
	url: 'https://google.com',
	vector: [0.1,0.1,0.1],
    prompt: 'a girl'
}).id;

-- 创建一个 id 为 item:1 的 item 记录
-- 并将 $text $image 作为 record
CREATE item:1 CONTENT {
    text: $text,
    image: $image
};

-- 创建一个 text 和 image record 为空的 item 记录
CREATE item:2 CONTENT {
    text: [],
    image: []
};

-- 创建 "包含" 关联
-- item:1 包含 text:aaa, image:bbb
RELATE item:1->contains->[text:aaa, image:bbb];

-- 常规 sql 查询
SELECT * from item:1;

-- 关联查询，正向
-- 查询 item:1 包含的 image 记录
SELECT ->contains->image from item:1;
-- 查询 item:1 包含的 text 记录
SELECT ->contains->text from item:1;

-- 反向查询，反向
-- 查询包含 text:aaa 的 item
SELECT <-contains<-item from text:aaa;
-- 查询包含 image:bbb 的 item
SELECT <-contains<-item from image:bbb;

-- 常规 sql 查询
-- [
-- 	{
-- 		id: item:1
-- 	}
-- ]
SELECT id FROM item;

-- VALUE 关键字查询
-- [
-- 	item:1
-- ]
SELECT VALUE id FROM item;

-- 包含 record 记录的查询
-- SELECT * FROM item:1 FETCH text, image;
```
