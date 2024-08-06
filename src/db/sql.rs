pub const CREATE_ITEM_TABLE: &str = r#"
-- 创建 "item" 表
DEFINE TABLE IF NOT EXISTS item;
-- 定义 "item" 表的字段
DEFINE FIELD IF NOT EXISTS id ON TABLE item TYPE string DEFAULT rand::uuid::v7();
DEFINE FIELD IF NOT EXISTS text ON TABLE item TYPE array<record<text>>;
DEFINE FIELD IF NOT EXISTS image ON TABLE item TYPE array<record<image>>;

-- 创建 "text" 表
DEFINE TABLE IF NOT EXISTS text;
-- 定义 "text" 表的字段
DEFINE FIELD IF NOT EXISTS id ON TABLE text TYPE string DEFAULT rand::uuid::v7();
DEFINE FIELD IF NOT EXISTS data ON TABLE text TYPE string;
DEFINE FIELD IF NOT EXISTS vector ON TABLE text TYPE array;

-- 创建 "image" 表
DEFINE TABLE IF NOT EXISTS image;
-- 定义 "image" 表的字段
DEFINE FIELD IF NOT EXISTS id ON TABLE image TYPE string DEFAULT rand::uuid::v7();
DEFINE FIELD IF NOT EXISTS url ON TABLE image TYPE string;
DEFINE FIELD IF NOT EXISTS vector ON TABLE image TYPE array;
DEFINE FIELD IF NOT EXISTS prompt ON TABLE image TYPE string;

-- 为每个表的 "id" 字段创建唯一索引
DEFINE INDEX IF NOT EXISTS idx_item_id ON item COLUMNS id UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_text_id ON text COLUMNS id UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_image_id ON image COLUMNS id UNIQUE;
    "#;
