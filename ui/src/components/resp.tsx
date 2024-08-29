import useStore from "@/store";
import {Accordion, AccordionContent, AccordionItem, AccordionTrigger} from "@/components/ui/accordion.tsx";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {SearchResultItem} from "@/types.ts";
import Gallery from "@/components/gallery.tsx";
import {useEffect, useState} from "react";

export default function ResponseWidget() {
    const { resp } = useStore();
    const [value, setValue] = useState<string[]>([]);

    useEffect(() => {
        setValue(resp.map((_, index) => index.toString()))
    }, [resp.length])

    const typeTips = (type: 'text' | 'image' | 'item') => {
        switch (type) {
            case 'text':
                return '文本';
            case 'image':
                return '图片';
            case 'item':
                return '组合';
        }
    }

    return <Accordion value={value} onValueChange={setValue} type="multiple" className="w-full">
        {
            resp.map((item, index) => (
                <AccordionItem value={index.toString()}>
                    <AccordionTrigger>
                        第 {index + 1} 项，{typeTips(item.type)}
                    </AccordionTrigger>
                    <AccordionContent className="space-y-4">
                        <ContentWidget {...item} />
                    </AccordionContent>
                </AccordionItem>
            ))
        }
    </Accordion>
}

const ContentWidget = (content: SearchResultItem) => {
    switch (content.type) {
        case 'text':
            return <Card>
                <CardHeader>
                    <CardTitle>{content.value.id}</CardTitle>
                </CardHeader>
                <CardContent className="space-y-2">{content.value.data}</CardContent>
            </Card>
        case 'image':
            return <Gallery images={[content.value]}/>
        case 'item':
            const { text, image } = content.value
            return <Card>
                <CardHeader>
                    <CardTitle>{content.value.id}</CardTitle>
                </CardHeader>
                <CardContent className="space-y-2 flex flex-col items-center">
                    {text.map((t, index) => (
                        <div key={index}>{t.data}</div>
                    ))}
                    {
                        image.length > 0 && <Gallery images={image}/>
                    }
                </CardContent>
            </Card>
    }
}
