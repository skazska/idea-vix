import type { Accessor, JSX } from "solid-js";


// type ListItemPart<I extends object, K extends keyof I = keyof I> = Record<K, 
type ListItemGetter = <I extends object, T extends readonly I[], U extends JSX.Element>( item: Accessor<T[number]>, index: number) => U;

