import {Card, CardContent, CardDescription, CardHeader, CardTitle,} from "./components/ui/card";
import {Tabs, TabsContent, TabsList, TabsTrigger} from "./components/ui/tabs";
import {useMemo} from "react";
import {Accordion, AccordionContent, AccordionItem, AccordionTrigger,} from "./components/ui/accordion";
import {cn} from "@/lib/utils.ts";
import ImageWidget from "@/components/image.tsx";
import useStore, {ActionType} from "./store";
import {LoaderCircle} from "lucide-react";
import AddWidget from "@/components/action.tsx";
import TextWidget from "@/components/text.tsx";
import Gallery from "@/components/gallery.tsx";
import ItemWidget from "./components/item";

function App() {
  const { resp, isLoading, action } = useStore();
  
  const isSearch = useMemo(() => action === ActionType.Search, [action]);

  const hasText = useMemo(() => (resp?.text ?? []).length > 0, [resp?.text]);
  const hasImage = useMemo(() => (resp?.image ?? []).length > 0, [resp?.image]);
  const hasItem = useMemo(() => (resp?.item ?? []).length > 0, [resp?.item]);

  return (
      <div className="relative w-screen h-screen">
        <div className="flex flex-col justify-start items-center bg-backgroud gap-8 p-8">
          <h1 className="font-bold text-3xl">Search Demo</h1>
          <AddWidget/>
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
                  <CardDescription>{isSearch ? 'Search' : 'Add'} with Text</CardDescription>
                </CardHeader>
                <CardContent className="space-y-2">
                  <TextWidget />
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="image">
              <Card>
                <CardHeader>
                  <CardTitle>Image</CardTitle>
                  <CardDescription>{isSearch ? 'Search' : 'Add'} with Image</CardDescription>
                </CardHeader>
                <CardContent className="space-y-2">
                  <ImageWidget />
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="item">
              <Card>
                <CardHeader>
                  <CardTitle>Item</CardTitle>
                  <CardDescription>{isSearch ? 'Search' : 'Add'} with Item</CardDescription>
                </CardHeader>
                <CardContent className="space-y-2">
                  <ItemWidget/>
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
          <div className="max-w-2xl w-full">
            <p className="text-muted-foreground">响应</p>
            <Accordion type="multiple" className="w-full">
              <AccordionItem value="text" disabled={!hasText}>
                <AccordionTrigger
                    className={cn(!hasText && "line-through text-muted-foreground")}
                >
                  文本
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  {resp?.text.map((item, index) => (
                      <Card key={index}>
                        <CardHeader>
                          <CardTitle>{item.id}</CardTitle>
                        </CardHeader>
                        <CardContent className="space-y-2">{item.data}</CardContent>
                      </Card>
                  ))}
                </AccordionContent>
              </AccordionItem>
              <AccordionItem value="image" disabled={!hasImage}>
                <AccordionTrigger
                    className={cn(!hasImage && "line-through text-muted-foreground")}
                >
                  图片
                </AccordionTrigger>
                <AccordionContent className="flex justify-center">
                  <Gallery images={resp?.image ?? []}/>
                </AccordionContent>
              </AccordionItem>
              <AccordionItem value="item" disabled={!hasItem}>
                <AccordionTrigger
                    className={cn(!hasItem && "line-through text-muted-foreground")}
                >
                  组合
                </AccordionTrigger>
                <AccordionContent className="space-y-4">
                  {resp?.item.map((item, index) => (
                      <Card key={index}>
                        <CardHeader>
                          <CardTitle>{item.id}</CardTitle>
                        </CardHeader>
                        <CardContent className="space-y-2 flex flex-col items-center">
                          {item.text.map((text, index) => (
                              <div key={index}>{text.data}</div>
                          ))}
                          {
                            item.image.length > 0 && <Gallery images={item.image}/>
                          }
                        </CardContent>
                      </Card>
                  ))}
                </AccordionContent>
              </AccordionItem>
            </Accordion>
          </div>
        </div>
        {
            isLoading && <div className="z-[100] fixed top-0 w-full h-full bg-black/80 cursor-progress">
              <div
                  className="absolute translate-x-[-50%] translate-y-[-50%] top-1/2 left-1/2 flex flex-col gap-4 items-center">
                <LoaderCircle className="animate-spin text-white"/>
                <p className="text-white text-sm">加载中</p>
              </div>
            </div>
        }
      </div>
  );
}

export default App;
