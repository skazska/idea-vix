import { createContext, createSignal, useContext, type Accessor, type ParentComponent, type Setter } from "solid-js";

type TPageState = {
    title: string;
    sessionAddress?: string;
    sessionToken?: string;
}

type TPageStateAccessor = {
    title: Accessor<string>;
    sessionAddress: Accessor<string | undefined>;
    sessionToken: Accessor<string | undefined>;
}

type TPageStateModel = [
    TPageStateAccessor,
    {
        setTitle: Setter<string>;
        setSessionAddress: Setter<string | undefined>;
        setSessionToken: Setter<string | undefined>;
    }
];

const PageStateContext = createContext<TPageStateModel>();

export const PageStateProvider: ParentComponent<TPageState> = (props) => {
    const [title, setTitle] = createSignal(props.title);
    const [sessionAddress, setSessionAddress] = createSignal(props.sessionAddress);
    const [sessionToken, setSessionToken] = createSignal(props.sessionToken);

    const model: TPageStateModel = [
        { title, sessionAddress, sessionToken },
        {
            setTitle,
            setSessionAddress,
            setSessionToken
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
