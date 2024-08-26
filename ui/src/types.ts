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
