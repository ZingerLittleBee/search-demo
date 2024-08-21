import { useCallback, useState } from "react";
import { Label } from "./ui/label";
import { Input } from "./ui/input";
import { toast } from "sonner";
import { Button } from "./ui/button";
import useUpload from "@/hook/useUpload";
import useSearch from "@/hook/useSearch";
import useStore from "@/store";

export function ImageUpload() {
  const [file, setFile] = useState<File | null>(null);
  const { uploadImage } = useUpload();
  const { searchWithImage } = useSearch();
  const { setResp } = useStore();
  const [url, setUrl] = useState<string>();

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = event.target.files?.[0];

    if (selectedFile) {
      if (selectedFile.type.startsWith("image/")) {
        setFile(selectedFile);
      } else {
        toast.error("Please select an image file.");
        event.target.value = "";
      }
    }
  };

  const handleSearch = useCallback(async () => {
    if (file) {
      const urls = await uploadImage(file);
      if (urls.length > 0) {
        setUrl(urls[0]);
        const resp = await searchWithImage(urls[0]);
        setResp(resp);
      }
    }
  }, [file, searchWithImage, setResp, uploadImage]);

  return (
    <div className="space-y-4">
      <div className="grid w-full max-w-sm items-center gap-1.5">
        <img src={url} className="w-full h-full object-contain" />
        <Input
          id="picture"
          type="file"
          accept="image/*"
          onChange={handleFileChange}
        />
      </div>
      <Button onClick={handleSearch}>查询</Button>
    </div>
  );
}
