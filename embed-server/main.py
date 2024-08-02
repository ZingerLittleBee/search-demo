from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List

from embed import image_to_vector, text_to_vector

app = FastAPI()


class ImageInput(BaseModel):
    base64_images: List[str]


class TextInput(BaseModel):
    texts: List[str]

@app.get("/")
async def root():
    return {"message": "Hello World"}


@app.post("/image_to_vector")
async def _image_to_vector(input: ImageInput):
    try:
        vectors = image_to_vector(input.base64_images)
        return {"vectors": vectors.tolist()}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/text_to_vector")
async def _text_to_vector(input: TextInput):
    try:
        vectors = text_to_vector(input.texts)
        return {"vectors": vectors.tolist()}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
