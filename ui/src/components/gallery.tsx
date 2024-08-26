import {ImageResult} from "@/hook/useSearch.ts";
import {Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious} from "@/components/ui/carousel.tsx";
import {Card, CardContent} from "@/components/ui/card.tsx";
import {Dialog, DialogContent, DialogTrigger } from "./ui/dialog";

export default function Gallery({images}: { images: ImageResult[]}) {
    return <Carousel className="w-full max-w-xs">
        <CarouselContent>
            {images.map((image, index) => (
                <CarouselItem key={index}>
                    <div className="p-1">
                        <Card>
                            <CardContent className="flex aspect-square items-center justify-center p-6">
                                <Dialog>
                                    <DialogTrigger asChild>
                                        <img src={image.url} alt={image.id}
                                             className="w-full h-full object-contain"/>
                                    </DialogTrigger>
                                    <DialogContent className="sm:max-w-[425px]">
                                        <img src={image.url} alt={image.id}
                                             className="w-full h-full object-contain"/>
                                    </DialogContent>
                                </Dialog>

                            </CardContent>
                        </Card>
                    </div>
                </CarouselItem>
            ))}
        </CarouselContent>
        <CarouselPrevious />
        <CarouselNext />
    </Carousel>
}
