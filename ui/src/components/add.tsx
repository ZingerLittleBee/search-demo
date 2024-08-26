import { Button } from "./ui/button";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";
import {useCallback, useState} from "react";
import useStore, {ActionType} from "@/store";
import {Tabs, TabsContent, TabsList, TabsTrigger} from "@/components/ui/tabs.tsx";
import {Card, CardContent, CardDescription, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import TextWidget from "@/components/text.tsx";
import ImageWidget from "@/components/image.tsx";
import ItemWidget from "./item";

export default function AddWidget() {
    const [open, setOpen] = useState(false)
    const { setAction } = useStore()

    const onOpenChange = useCallback((e: boolean) => {
        setOpen(e)
        setAction(e ? ActionType.Add : ActionType.Search)
    }, [setOpen])

    return <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogTrigger asChild>
            <div>
            <Button variant="outline">添加数据</Button>
            </div>
        </DialogTrigger>
        <DialogContent className="sm:max-w-[425px]">
            <DialogHeader>
                <DialogTitle>Add Data</DialogTitle>
            </DialogHeader>
            <Tabs defaultValue="text" className="w-full max-w-2xl ">
                <TabsList className="grid w-full grid-cols-3">
                    <TabsTrigger value="text">文本</TabsTrigger>
                    <TabsTrigger value="image">图片</TabsTrigger>
                    <TabsTrigger value="item">组合</TabsTrigger>
                </TabsList>
                <TabsContent value="text">
                    <Card>
                        <CardHeader>
                            <CardTitle>Text</CardTitle>
                            <CardDescription>Search with Text</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-2">
                            <TextWidget/>
                        </CardContent>
                    </Card>
                </TabsContent>
                <TabsContent value="image">
                    <Card>
                        <CardHeader>
                            <CardTitle>Image</CardTitle>
                            <CardDescription>Search with Image</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-2">
                            <ImageWidget/>
                        </CardContent>
                    </Card>
                </TabsContent>
                <TabsContent value="item">
                    <Card>
                        <CardHeader>
                            <CardTitle>Item</CardTitle>
                            <CardDescription>Search with Item</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-2 max-h-[600px] overflow-y-auto">
                            <ItemWidget />
                        </CardContent>
                    </Card>
                </TabsContent>
            </Tabs>
        </DialogContent>
    </Dialog>
}
