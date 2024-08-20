import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './components/ui/card'
import { Label } from './components/ui/label'
import { Input } from './components/ui/input'
import { Button } from './components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from './components/ui/tabs'
import useSearch, {SearchResult} from "@/hook/useSearch.ts";
import {useCallback, useMemo, useState} from "react";
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from './components/ui/accordion'
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from './components/ui/carousel'
import {cn} from "@/lib/utils.ts";

function App() {
    const [text, setText] = useState('')
    const [resp, setResp] = useState<SearchResult>()

    console.log('resp', resp, resp?.image.length)

    const { searchWithText } = useSearch()

    const handleSearchText = useCallback(async () => {
        if (text) {
            const resp = await searchWithText(text)
            setResp(resp)
        }
    }, [text])

    const hasImage = useMemo(() => (resp?.image.length ?? 0) > 0, [resp])

  return (
    <div className="w-screen h-screen flex flex-col justify-start bg-backgroud gap-8 p-8">
        <Tabs defaultValue="text" className="w-full max-w-[600px]">
            <TabsList className="grid w-full grid-cols-3">
                <TabsTrigger value="text">文本</TabsTrigger>
                <TabsTrigger value="image">图片</TabsTrigger>
                <TabsTrigger value="item">组合</TabsTrigger>
            </TabsList>
            <TabsContent value="text">
                <Card>
                    <CardHeader>
                        <CardTitle>Account</CardTitle>
                        <CardDescription>
                            Make changes to your account here. Click save when you're done.
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-2">
                        <div className="space-y-1">
                            <Label htmlFor="text">Text</Label>
                            <Input id="text" value={text} onChange={(e) => setText(e.target.value)} />
                        </div>
                    </CardContent>
                    <CardFooter>
                        <Button onClick={handleSearchText}>请求</Button>
                    </CardFooter>
                </Card>
            </TabsContent>
            <TabsContent value="image">
                <Card>
                    <CardHeader>
                        <CardTitle>Password</CardTitle>
                        <CardDescription>
                            Change your password here. After saving, you'll be logged out.
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-2">
                        <div className="space-y-1">
                            <Label htmlFor="current">Current password</Label>
                            <Input id="current" type="password" />
                        </div>
                        <div className="space-y-1">
                            <Label htmlFor="new">New password</Label>
                            <Input id="new" type="password" />
                        </div>
                    </CardContent>
                    <CardFooter>
                        <Button>Save password</Button>
                    </CardFooter>
                </Card>
            </TabsContent>
            <TabsContent value="item">
                <Card>
                    <CardHeader>
                        <CardTitle>Password</CardTitle>
                        <CardDescription>
                            Change your password here. After saving, you'll be logged out.
                        </CardDescription>
                    </CardHeader>
                    <CardContent className="space-y-2">
                        <div className="space-y-1">
                            <Label htmlFor="current">Current password</Label>
                            <Input id="current" type="password" />
                        </div>
                        <div className="space-y-1">
                            <Label htmlFor="new">New password</Label>
                            <Input id="new" type="password" />
                        </div>
                    </CardContent>
                    <CardFooter>
                        <Button>Save password</Button>
                    </CardFooter>
                </Card>
            </TabsContent>
        </Tabs>
        <div>
            <p className="text-muted-foreground">响应</p>
            <Accordion type="multiple" className="w-full">
            <AccordionItem value="text" disabled={!resp?.text}>
                <AccordionTrigger className={cn(!resp?.text && 'line-through text-muted-foreground')}>
                    文本
                </AccordionTrigger>
                <AccordionContent>
                    {
                        resp?.text.map((item, index) => (
                            <Card key={index}>
                                <CardHeader>
                                <CardTitle>{item.id}</CardTitle>
                                </CardHeader>
                                <CardContent className="space-y-2">
                                    {item.data}
                                </CardContent>
                            </Card>
                        ))
                    }
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="image" disabled={!hasImage}>
                <AccordionTrigger className={cn(!hasImage && 'line-through text-muted-foreground')}>
                        图片
                </AccordionTrigger>
                <AccordionContent className="flex justify-center">
                    <Carousel className="w-full max-w-xs">
                        <CarouselContent>
                            {resp?.image.map((image, index) => (
                                <CarouselItem key={index}>
                                    <div className="p-1">
                                        <Card>
                                            <CardContent className="flex aspect-square items-center justify-center p-6">
                                                <img src={image.url} alt={image.id} className="w-full h-full object-cover" />
                                            </CardContent>
                                        </Card>
                                    </div>
                                </CarouselItem>
                            ))}
                        </CarouselContent>
                        <CarouselPrevious />
                        <CarouselNext />
                    </Carousel>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem value="item-3">
                <AccordionTrigger>组合</AccordionTrigger>
                <AccordionContent>
                    Yes. It's animated by default, but you can disable it if you prefer.
                </AccordionContent>
            </AccordionItem>
        </Accordion>
        </div>
    </div>
  )
}

export default App
