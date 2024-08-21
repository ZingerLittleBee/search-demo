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
import useSearch, { SearchResult } from "@/hook/useSearch.ts";
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

function App() {
  const [text, setText] = useState("");
  const [resp, setResp] = useState<SearchResult>();

  const { searchWithText } = useSearch();

  const handleSearchText = useCallback(async () => {
    if (text) {
      const resp = await searchWithText(text);
      setResp(resp);
    }
  }, [text, searchWithText]);

  const hasImage = useMemo(() => (resp?.image.length ?? 0) > 0, [resp]);

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
              <CardTitle>Account</CardTitle>
              <CardDescription>
                Make changes to your account here. Click save when you're done.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-2">
              <div className="space-y-1">
                <Label htmlFor="text">Text</Label>
                <Input
                  id="text"
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                />
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
              <CardTitle>Image</CardTitle>
              <CardDescription>
                Change your password here. After saving, you'll be logged out.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-2">
              <ImageUpload />
            </CardContent>
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
      <div className="w-full max-w-2xl">
        <p className="text-muted-foreground">响应</p>
        <Accordion type="multiple" className="w-full">
          <AccordionItem value="text" disabled={!resp?.text}>
            <AccordionTrigger
              className={cn(
                !resp?.text && "line-through text-muted-foreground",
              )}
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
          <AccordionItem value="item-3">
            <AccordionTrigger>组合</AccordionTrigger>
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
