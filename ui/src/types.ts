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

export type SearchResultItem = {
    type: 'text'
    value: TextResult
} | {
    type: 'image'
    value: ImageResult
} | {
    type: 'item'
    value: ItemResult
}

export type SearchResults = SearchResultItem[]

export type Result<T = any> = {
    data: T
    message?: string
}
