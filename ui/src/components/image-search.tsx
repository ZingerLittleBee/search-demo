import { useCallback, useState } from "react";
import { Label } from "./ui/label";
import { Input } from "./ui/input";
import { toast } from "sonner";
import { Button } from "./ui/button";
import useUpload from "@/hook/useUpload";
import useSearch from "@/hook/useSearch";

export function ImageUpload() {
  const [file, setFile] = useState<File | null>(null);
  const { uploadImage } = useUpload();
  const { searchWithImage } = useSearch();

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
      const url = await uploadImage(file);
      if (url) {
        await searchWithImage(url);
      }
    }
  }, [file, searchWithImage, uploadImage]);

  return (
    <div className="space-y-4">
      <div className="grid w-full max-w-sm items-center gap-1.5">
        <Label htmlFor="picture">Picture</Label>
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
