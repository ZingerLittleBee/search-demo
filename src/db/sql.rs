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
-- image vector
DEFINE FIELD IF NOT EXISTS vector ON TABLE image TYPE array;
DEFINE FIELD IF NOT EXISTS prompt ON TABLE image TYPE string;
DEFINE FIELD IF NOT EXISTS prompt_vector ON TABLE image TYPE array;

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

-- 定义向量索引
DEFINE INDEX IF NOT EXISTS idx_text_vector_hnsw_d512 ON text FIELDS vector HNSW DIMENSION 512 DIST EUCLIDEAN;
DEFINE INDEX IF NOT EXISTS idx_image_prompt_vector_hnsw_d512 ON image FIELDS prompt_vector HNSW DIMENSION 512 DIST EUCLIDEAN;
DEFINE INDEX IF NOT EXISTS idx_image_vector_hnsw_d512 ON image FIELDS vector HNSW DIMENSION 512 DIST EUCLIDEAN;

-- 定义分词器
-- https://github.com/surrealdb/surrealdb/issues/2850
DEFINE ANALYZER IF NOT EXISTS mixed_analyzer TOKENIZERS blank, class, punct FILTERS lowercase, ascii, snowball(english);

-- 定义索引
DEFINE INDEX IF NOT EXISTS mixed_index_text_data ON text FIELDS data SEARCH ANALYZER mixed_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX IF NOT EXISTS mixed_index_image_prompt ON image FIELDS prompt SEARCH ANALYZER mixed_analyzer BM25 HIGHLIGHTS;
"#;
