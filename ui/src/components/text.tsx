import {Input} from "@/components/ui/input.tsx";
import {useCallback, useState} from "react";
import useStore, {ActionType} from "@/store";
import useSearch from "@/hook/useSearch.ts";
import {Button} from "@/components/ui/button.tsx";
import useAdd from "@/hook/useAdd.ts";

export default function TextWidget() {
    const [text, setText] = useState("");
    const { setResp, action } = useStore();
    const { searchWithText } = useSearch();
    const { addText } = useAdd()

    const handleTextAction = useCallback(async () => {
        if (text) {
            if (action === ActionType.Search) {
                const resp = await searchWithText(text);
                setResp(resp);
            } else {
                await addText(text)
            }
        }
    }, [text, searchWithText, setResp]);

    return <div className="space-y-4">
        <Input
            id="text"
            value={text}
            onChange={(e) => setText(e.target.value)}
        />
        <Button onClick={handleTextAction}>{
            action === ActionType.Search ? "查询" : "添加"
        }</Button>
    </div>
}
