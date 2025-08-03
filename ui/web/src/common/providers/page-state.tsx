import { createContext, createSignal, useContext, type Accessor, type ParentComponent, type Setter } from "solid-js";

type TPageState = {
    title: string;
    sessionAddress?: string;
    sessionOk: boolean;
    sessionExpiresAt?: number;
}

type TPageStateAccessor = {
    title: Accessor<string>;
    sessionOk: Accessor<boolean>;
    sessionAddress: Accessor<string | undefined>;
    sessionExpiresAt: Accessor<number | undefined>;
}

type TPageStateModel = [
    TPageStateAccessor,
    {
        setTitle: Setter<string>;
        setSessionOk: Setter<boolean>;
        setSessionAddress: Setter<string | undefined>;
        setSessionExpiresAt: Setter<number | undefined>;
    }
];

const PageStateContext = createContext<TPageStateModel>();

export const PageStateProvider: ParentComponent<TPageState> = (props) => {
    const [title, setTitle] = createSignal(props.title);
    const [sessionAddress, setSessionAddress] = createSignal(props.sessionAddress);
    const [sessionExpiresAt, setSessionExpiresAt] = createSignal(props.sessionExpiresAt);
    const [sessionOk, setSessionOk] = createSignal(false);

    const model: TPageStateModel = [
        { title, sessionAddress, sessionExpiresAt, sessionOk },
        {
            setTitle,
            setSessionAddress,
            setSessionExpiresAt,
            setSessionOk
        }
    ]

    return (
        <PageStateContext.Provider value={model}>
            {props.children}
        </PageStateContext.Provider>
    );
}

export function usePageState() {
    if (!PageStateContext) {
        console.error("usePageState called outside of PageStateProvider");
        throw new Error("usePageState must be used within a PageStateProvider");
    }

    const context = useContext(PageStateContext);

    if (!context) {
        console.error("usePageState context is empty");
        throw new Error("usePageState must be used within a PageStateProvider");
    }

    return context;
}

export function setTitle(title: string) {
    const [_, setPageState] = usePageState();
    setPageState.setTitle(title);
}

export function setSessionExpiresAt(expiresAt: number | undefined) {
    const [_, setPageState] = usePageState();
    setPageState.setSessionExpiresAt(expiresAt);
}
