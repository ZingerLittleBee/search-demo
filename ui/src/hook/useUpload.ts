import { toast } from "sonner";
import { Result } from "./useSearch";

export default function useUpload() {
  const uploadImage = async (files: File[]): Promise<string[]> => {
    try {
      const formData = new FormData();
      files.forEach((file, index) => {
        formData.append(`file-${index}`, file, file.name);
      });

      const resp = await fetch("/api/upload/image", {
        method: "POST",
        body: formData,
      });

      const result: Result<string[]> = await resp.json();

      if (result.message) {
        toast(result.message);
        return [];
      }

      return result.data ?? [];
    } catch (e) {
      console.error(e);
      toast(`Failed to upload image: ${files.map((file) => file.name).join(", ")}`);
    }
    return [];
  };

  return {
    uploadImage,
  };
}
