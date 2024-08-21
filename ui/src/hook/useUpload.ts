import { toast } from "sonner";
import { Result } from "./useSearch";

export type UploadResult = {
  url: string;
};

export default function useUpload() {
  const uploadImage = async (file: File): Promise<string | undefined> => {
    try {
      const formData = new FormData();
      formData.append("image", file);

      const resp = await fetch("/api/upload/image", {
        method: "POST",
        body: formData,
      });

      const result: Result<UploadResult> = await resp.json();

      if (result.message) {
        toast(result.message);
        return;
      }

      return result.data.url;
    } catch (e) {
      console.error(e);
      toast(`Failed to upload image: ${file.name}`);
    }
  };

  return {
    uploadImage,
  };
}
