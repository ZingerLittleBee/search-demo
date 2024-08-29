import { toast } from "sonner"
import useStore from "@/store";
import {Result, SearchResults} from "@/types.ts";

export default function useSearch() {
    const { setIsLoading } = useStore()

    const searchWithText = async (text: string): Promise<SearchResults> => {
        setIsLoading(true)
        try {
            const resp = await fetch('/api/search/text', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ text })
            })
            const result: Result<SearchResults> = await resp.json()

            if (result.message) {
                toast(result.message)
                setIsLoading(false)
                return []
            }

            setIsLoading(false)
            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with text: ${text}`)
            setIsLoading(false)
        }
        return []
    }

    const searchWithImage = async (url: string): Promise<SearchResults> => {
        setIsLoading(true)
        try {
            const resp = await fetch('/api/search/image', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ url })
            })
            const result: Result<SearchResults> = await resp.json()

            if (result.message) {
                toast(result.message)
                setIsLoading(false)
                return []
            }

            setIsLoading(false)
            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with image with url: ${url}`)
            setIsLoading(false)
        }
        return []
    }

    const searchWithItem = async (item: { text: string[]; image: string[] }): Promise<SearchResults> => {
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
            const result: Result<SearchResults> = await resp.json()

            if (result.message) {
                toast(result.message)
                setIsLoading(false)
                return []
            }

            setIsLoading(false)
            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with item: ${item}`)
            setIsLoading(false)
        }
        return []
    }

    return {
        searchWithText,
        searchWithImage,
        searchWithItem,
    }
}
