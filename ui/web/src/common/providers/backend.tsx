/* @refresh reload */
interface IResp {
    headers: Headers;
    status: number;
    data?: unknown;
    ok: boolean;
}

interface IError {
    error: Error;
    ok: false;
}
export type OrError<T> = T | IError;

export interface IBackend {
    fetchJson: (url: string, options?: RequestInit) => Promise<OrError<IResp>>;
};
import { createContext, useContext, type Accessor } from "solid-js";
import { usePageState } from "./page-state";

class Backend {
    private userId: Accessor<string>

    constructor(userId: Accessor<string>) {
        this.userId = userId;
    }

    private getFetchJSONOptions(options: RequestInit = {}): RequestInit {
        return {
            ...options,
            headers: {
                "Content-Type": "application/json",
                "Accept": "application/json",
                "X-User-Id": this.userId(),
                ...options.headers,
            },
        }
    }

    public async fetchJson(url: string, options?: RequestInit): Promise<OrError<IResp>> {
        const response = await fetch(url, this.getFetchJSONOptions(options));
        try {
            const data = response.ok ? await response.json() : await response.text();
            return { data, status: response.status, headers: response.headers, ok: response.ok};
        } catch (error) {
            console.error("Failed to parse JSON response:", error);
            return { error: new Error(`Failed to parse JSON response from ${url}`), ok: false };
        }
    }
}

const BackendContext = createContext<IBackend>();

export async function getResponse<T>(
    fetcherPromise: Promise<OrError<IResp>>,
    getData: (input: unknown) => T | Promise<T>,
    errorDetils?: string
): Promise<T> {
    const response = await fetcherPromise;

    if (!response.ok) {
        throw 'error' in response
            ? response.error 
            : new Error(`Failed to fetch: ${response.status} ${response.data} ${errorDetils ? `(${errorDetils})` : ''}`);
    }

    return getData(await response.data);
}

export const BackendProvider = (props: { children: any }) => {
    const [{ userId }] = usePageState();

    const backend: IBackend = new Backend(userId);

    console.log("BackendProvider rendered");

    return (
        <BackendContext.Provider value={backend}>
            {props.children}
        </BackendContext.Provider>
    )
}

export function useBackend(): IBackend {
    if (!BackendContext) {
        console.error("useBackend called outside of BackendProvider");
        throw new Error("no context: useBackend must be used within a BackendProvider");
    }

    const context = useContext(BackendContext);

    if (!context) {
        console.error("useBackend context is empty");
        throw new Error("empty context: useBackend must be used within a BackendProvider");
    }

    return context;
}   