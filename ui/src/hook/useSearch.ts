import { toast } from "sonner"

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

    const searchWithText = async (text: string): Promise<SearchResult | undefined> => {
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
                return
            }

            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with text: ${text}`)
        }
    }

    const searchWithImage = async (url: string): Promise<SearchResult | undefined> => {
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
                return
            }

            return result.data
        } catch (e) {
            console.error(e)
            toast(`Failed to search with image with url: ${url}`)
        }
    }

    return {
        searchWithText,
        searchWithImage
    }
}
