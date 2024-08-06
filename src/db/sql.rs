pub const CREATE_TABLE: &str = r#"
-- 创建 "item" 表
DEFINE TABLE IF NOT EXISTS item;
-- 定义 "item" 表的字段
DEFINE FIELD IF NOT EXISTS text ON TABLE item TYPE array<record<text>>;
DEFINE FIELD IF NOT EXISTS image ON TABLE item TYPE array<record<image>>;

-- 创建 "text" 表
DEFINE TABLE IF NOT EXISTS text;
-- 定义 "text" 表的字段
DEFINE FIELD IF NOT EXISTS data ON TABLE text TYPE string;
DEFINE FIELD IF NOT EXISTS vector ON TABLE text TYPE array;

-- 创建 "image" 表
DEFINE TABLE IF NOT EXISTS image;
-- 定义 "image" 表的字段
DEFINE FIELD IF NOT EXISTS url ON TABLE image TYPE string;
DEFINE FIELD IF NOT EXISTS vector ON TABLE image TYPE array;
DEFINE FIELD IF NOT EXISTS prompt ON TABLE image TYPE string;

-- 创建 "frame" 表
DEFINE TABLE IF NOT EXISTS frame;
-- 定义 "frame" 表的字段
DEFINE FIELD IF NOT EXISTS point ON TABLE frame TYPE datetime;
DEFINE FIELD IF NOT EXISTS text ON TABLE frame TYPE record<text>;
DEFINE FIELD IF NOT EXISTS image ON TABLE frame TYPE record<image>;

-- 创建 "video" 表
DEFINE TABLE IF NOT EXISTS video;
-- 定义 "video" 表的字段
DEFINE FIELD IF NOT EXISTS url ON TABLE video TYPE string;
DEFINE FIELD IF NOT EXISTS frame ON TABLE video TYPE array<record<frame>>;
"#;
