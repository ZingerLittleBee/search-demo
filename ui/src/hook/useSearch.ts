import { toast } from "sonner"
import useStore from "@/store";
import {Result, SearchResult} from "@/types.ts";

export default function useSearch() {
    const { setIsLoading } = useStore()

    const searchWithText = async (text: string): Promise<SearchResult | undefined> => {
        setIsLoading(true)
        try {
            const resp = await fetch('/api/search/text', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ text })
            })
            const result: Result<SearchResult> = await resp.json()

            if (result.message) {
                toast(result.message)
                setIsLoading(false)
                return
            }

            setIsLoading(false)
            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with text: ${text}`)
            setIsLoading(false)
        }
    }

    const searchWithImage = async (url: string): Promise<SearchResult | undefined> => {
        setIsLoading(true)
        try {
            const resp = await fetch('/api/search/image', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ url })
            })
            const result: Result<SearchResult> = await resp.json()

            if (result.message) {
                toast(result.message)
                setIsLoading(false)
                return
            }

            setIsLoading(false)
            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with image with url: ${url}`)
            setIsLoading(false)
        }
    }

    const searchWithItem = async (item: { text: string[]; image: string[] }): Promise<SearchResult | undefined> => {
        const { text, image } = item
        setIsLoading(true)
        try {
            const resp = await fetch('/api/search/item', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ text, image })
            })
            const result: Result<SearchResult> = await resp.json()

            if (result.message) {
                toast(result.message)
                setIsLoading(false)
                return
            }

            setIsLoading(false)
            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with item: ${item}`)
            setIsLoading(false)
        }
    }

    return {
        searchWithText,
        searchWithImage,
        searchWithItem,
    }
}
