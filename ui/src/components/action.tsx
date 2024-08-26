import {useCallback, useState} from "react";
import useStore, {ActionType} from "@/store";
import { ToggleGroup, ToggleGroupItem } from "./ui/toggle-group";

export default function AddWidget() {
    const { setAction } = useStore()
    const [value, setValue] = useState<string>("search")

    const onValueChange = useCallback((e: string) => {
        setValue(e)
        setAction(e === "search" ? ActionType.Search : ActionType.Add)
    }, [setValue, setAction])

    return <ToggleGroup type="single" defaultValue="search" value={value} onValueChange={onValueChange}>
        <ToggleGroupItem value="search" aria-label="Toggle search" className="w-40">
            查询
        </ToggleGroupItem>
        <ToggleGroupItem value="add" aria-label="Toggle add" className="w-40">
            入库
        </ToggleGroupItem>
    </ToggleGroup>
}
