import { useCallback, useState } from "react";
import { Input } from "./ui/input";
import { toast } from "sonner";
import { Button } from "./ui/button";
import useUpload from "@/hook/useUpload";
import useSearch from "@/hook/useSearch";
import useStore, {ActionType} from "@/store";
import {Label} from "@/components/ui/label.tsx";
import useAdd from "@/hook/useAdd.ts";

export default function ImageWidget() {
  const [file, setFile] = useState<File | null>(null);
  const { uploadImage } = useUpload();
  const { searchWithImage } = useSearch();
  const { addImage } = useAdd()

  const { setResp, action } = useStore();
  const [url, setUrl] = useState<string>();

  const handleImageChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = event.target.files?.[0];
    if (selectedFile) {
      if (selectedFile.type.startsWith("image/")) {
        setFile(selectedFile);
        setUrl(URL.createObjectURL(selectedFile));
      } else {
        toast.error("Please select an image file.");
        event.target.value = "";
      }
    }
  }, [setFile, setUrl])

  const handleSearch = useCallback(async () => {
    if (file) {
      const urls = await uploadImage([file]);
      if (urls.length > 0) {
        const url = urls[0];
        if (action === ActionType.Search) {
          const resp = await searchWithImage(url);
          setResp(resp);
        } else {
            await addImage(url)
        }
      }
    }
  }, [file, searchWithImage, setResp, uploadImage]);

  return (
      <div className="space-y-4">
        <div className="flex-grow">
          <Label htmlFor='image-search' className="cursor-pointer">
            {file ? (
                <img src={url} alt={url} className="w-full max-w-sm object-cover"/>
            ) : (
                <div className="border-2 border-dashed border-gray-300 rounded-md p-2 text-center">
                  点击上传图片
                </div>
            )}
          </Label>
          <Input
              id='image-search'
              type="file"
              accept="image/*"
              className="hidden"
              onChange={handleImageChange}
          />
        </div>

        <Button onClick={handleSearch}>{action === ActionType.Search ? "查询" : "添加"}</Button>
      </div>
  );
}
