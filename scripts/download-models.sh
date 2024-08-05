# bash scripts/download-models.sh
resources_dir="resources"

if [ ! -d "${resources_dir}/CLIP-ViT-B-32-multilingual-v1" ]; then
  curl -L "https://gendam.s3.us-west-1.amazonaws.com/models/CLIP-ViT-B-32-multilingual-v1.tar.gz" | tar xz -C "${resources_dir}/"
fi
