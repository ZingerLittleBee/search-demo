import {useCallback, useState} from 'react'
import {Button} from "@/components/ui/button"
import {Input} from "@/components/ui/input"
import {Label} from "@/components/ui/label"
import {MinusIcon, PlusIcon} from 'lucide-react'
import useSearch from "@/hook/useSearch.ts";
import useUpload from "@/hook/useUpload.ts";
import useStore, {ActionType} from "@/store";
import useAdd from "@/hook/useAdd.ts";

export default function ItemWidget() {
    const { searchWithItem } = useSearch()
    const { uploadImage } = useUpload()
    const { setResp, action } = useStore()
    const { addItem } = useAdd()

    const [textInputs, setTextInputs] = useState([''])
    const [imageInputs, setImageInputs] = useState<{
        file?: File
        url?: string
    }[]>([{}])

    const addInput = useCallback((type: 'text' | 'image') => {
        if (type === 'text') {
            setTextInputs(prev => [...prev, ''])
        } else {
            setImageInputs(prev => [...prev, {}])
        }
    }, [setTextInputs, setImageInputs])

    const removeInput = useCallback((type: 'text' | 'image', index: number) => {
        if (type === 'text') {
            setTextInputs(prev => prev.filter((_, i) => i !== index))
        } else {
            setImageInputs(prev => prev.filter((_, i) => i !== index))
        }
    }, [setTextInputs, setImageInputs])

    const handleTextChange = useCallback((index: number, value: string) => {
        setTextInputs(prev => prev.map((item, i) => i === index ? value : item))
    }, [setTextInputs])

    const handleImageChange = useCallback((index: number, file: File | null) => {
        setImageInputs(prev => prev.map((item, i) => i === index ? (file ? {file, url: URL.createObjectURL(file)} : {}) : item))
    }, [setImageInputs])

    const handleSubmit = useCallback(async (e: React.FormEvent) => {
        e.preventDefault()
        const text = textInputs.filter(Boolean)
        const imageFile = imageInputs.filter(item => item.file).map(item => item.file as File)
        let image: string[] = []
        if (imageFile.length > 0) {
            image.concat(await uploadImage(imageFile))
        }


        if (action === ActionType.Search) {
            const res = await searchWithItem({ text, image })
            setResp(res)
        } else {
            await addItem({ text, image })
        }

    }, [textInputs, imageInputs])

    return (
        <form onSubmit={handleSubmit} className="space-y-8 mx-auto">
            <div className="space-y-4">
                <Label>文本输入</Label>
                {textInputs.map((text, index) => (
                    <div key={index} className="flex items-center space-x-2">
                        <Input
                            type="text"
                            value={text}
                            onChange={(e) => handleTextChange(index, e.target.value)}
                            placeholder={`文本 ${index + 1}`}
                        />
                        <Button
                            type="button"
                            variant="outline"
                            size="icon"
                            onClick={() => removeInput('text', index)}
                            disabled={textInputs.length === 1}
                        >
                            <MinusIcon className="h-4 w-4" />
                        </Button>
                    </div>
                ))}
                <Button type="button" onClick={() => addInput('text')} className="mt-2">
                    <PlusIcon className="h-4 w-4 mr-2" />
                    添加文本输入
                </Button>
            </div>
            <div className="space-y-4">
                <Label>图片上传</Label>
                {imageInputs.map((image, index) => (
                    <div key={index} className="flex items-center space-x-2">
                        <div className="flex-grow">
                            <Label htmlFor={`image-${index}`} className="cursor-pointer">
                                {image.url ? (
                                    <img src={image.url} alt={`Uploaded ${index + 1}`} className="w-full max-w-sm object-cover" />
                                ) : (
                                    <div className="border-2 border-dashed border-gray-300 rounded-md p-2 text-center">
                                        点击上传图片
                                    </div>
                                )}
                            </Label>
                            <Input
                                id={`image-${index}`}
                                type="file"
                                accept="image/*"
                                className="hidden"
                                onChange={(e) => handleImageChange(index, e.target.files?.[0] || null)}
                            />
                        </div>
                        <Button
                            type="button"
                            variant="outline"
                            size="icon"
                            onClick={() => removeInput('image', index)}
                            disabled={imageInputs.length === 1}
                        >
                            <MinusIcon className="h-4 w-4" />
                        </Button>
                    </div>
                ))}
                <Button type="button" onClick={() => addInput('image')} className="mt-2">
                    <PlusIcon className="h-4 w-4 mr-2" />
                    添加图片上传
                </Button>
            </div>
            <div className="space-y-2">
                <Label className="text-muted-foreground">所有图片不超过 10 MB</Label>
                <Button type="submit" className="w-full">{action === ActionType.Search ? "查询" : "添加"}</Button>
            </div>
        </form>
    )
}
