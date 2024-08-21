import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { Label } from "./components/ui/label";
import { Input } from "./components/ui/input";
import { Button } from "./components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./components/ui/tabs";
import useSearch from "@/hook/useSearch.ts";
import { useCallback, useMemo, useState } from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "./components/ui/accordion";
import { cn } from "@/lib/utils.ts";
import ImageWidget from "@/components/image.tsx";
import { ImageUpload } from "./components/image-search";
import useStore from "./store";

function App() {
  const [text, setText] = useState("");
  const { resp, setResp } = useStore();

  const { searchWithText } = useSearch();

  const handleSearchText = useCallback(async () => {
    if (text) {
      const resp = await searchWithText(text);
      setResp(resp);
    }
  }, [text, searchWithText, setResp]);

  const hasText = useMemo(() => (resp?.text ?? []).length > 0, [resp?.text]);
  const hasImage = useMemo(() => (resp?.image ?? []).length > 0, [resp?.image]);
  const hasItem = useMemo(() => (resp?.item ?? []).length > 0, [resp?.item]);

  console.log("resp?.item", resp, hasItem);

  return (
    <div className="w-screen h-screen flex flex-col justify-start items-center bg-backgroud gap-8 p-8">
      <Tabs defaultValue="text" className="w-full max-w-2xl">
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
              <div className="space-y-1">
                <Input
                  id="text"
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                />
              </div>
            </CardContent>
            <CardFooter>
              <Button onClick={handleSearchText}>查询</Button>
            </CardFooter>
          </Card>
        </TabsContent>
        <TabsContent value="image">
          <Card>
            <CardHeader>
              <CardTitle>Image</CardTitle>
              <CardDescription>Search with Image</CardDescription>
            </CardHeader>
            <CardContent className="space-y-2">
              <ImageUpload />
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="item">
          <Card>
            <CardHeader>
              <CardTitle>Item</CardTitle>
              <CardDescription>Search with Item</CardDescription>
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
      <div className="w-full max-w-2xl">
        <p className="text-muted-foreground">响应</p>
        <Accordion type="multiple" className="w-full">
          <AccordionItem value="text" disabled={!hasText}>
            <AccordionTrigger
              className={cn(!hasText && "line-through text-muted-foreground")}
            >
              文本
            </AccordionTrigger>
            <AccordionContent>
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
              <ImageWidget images={resp?.image ?? []} />
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
                    <ImageWidget images={item?.image ?? []} />
                  </CardContent>
                </Card>
              ))}
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
    </div>
  );
}

export default App;
