import { toast } from "sonner"
import {useState} from "react";

export type TextResult = {
    id: string
    data: string
}

export type ImageResult = {
    id: string
    url: string
}

export type ItemResult = {
    id: string
    text: TextResult[]
    image: ImageResult[]
}

export type SearchResult = {
    text: TextResult[]
    image: ImageResult[]
    item: ItemResult[]
}

export type Result<T = any> = {
    data: T
    message?: string
}

export default function useSearch() {
    const [isLoading, setIsLoading] = useState(false)

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
        isLoading
    }
}
