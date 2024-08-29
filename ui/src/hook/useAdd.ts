import {Result} from "@/types.ts";
import {toast} from "sonner";
import useStore from "@/store";

export default function useAdd() {
    const { setIsLoading } = useStore()

    const addText = async (text: string): Promise<void> => {
        setIsLoading(true)
        try {
            const resp = await fetch('/api/inbound/text', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ text })
            })
            const result: Result = await resp.json()

            if (result.message) {
                toast(result.message)
            }
            setIsLoading(false)
            toast('添加成功')
        } catch (e) {
            console.error(e)
            toast(`Failed to inbound with text: ${text}`)
            setIsLoading(false)
        }
    }

    const addImage = async (url: string): Promise<void> => {
        setIsLoading(true)
        try {
            const resp = await fetch('/api/inbound/image', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ url })
            })
            const result: Result = await resp.json()

            if (result.message) {
                toast(result.message)
            }

            setIsLoading(false)
            toast('添加成功')
        } catch (e) {
            console.error(e)
            toast(`Failed to search with image with url: ${url}`)
            setIsLoading(false)
        }
    }

    const addItem = async (item: { text: string[]; image: string[] }): Promise<void> => {
        const { text, image } = item
        setIsLoading(true)
        try {
            const resp = await fetch('/api/inbound/item', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ text, image })
            })
            const result: Result = await resp.json()

            if (result.message) {
                toast(result.message)
            }
            setIsLoading(false)
            toast('添加成功')
        } catch (e) {
            console.error(e)
            toast(`Failed to search with item: ${item}`)
            setIsLoading(false)
        }
    }

    return {
        addText,
        addImage,
        addItem
    }
}
