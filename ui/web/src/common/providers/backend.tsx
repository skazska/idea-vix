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
import { createContext, useContext, onMount } from "solid-js";
import { usePageState } from "./page-state";
import type { TSessionData } from "../../session/model";

class Backend {
    private getFetchJSONOptions(options: RequestInit = {}): RequestInit {
        return {
            ...options,
            // Include credentials to allow cookies to be sent in cross-origin requests
            credentials: 'include',
            headers: {
                "Content-Type": "application/json",
                "Accept": "application/json",
                // Token is sent via HttpOnly cookie now
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
    const [_pageState, setPageState] = usePageState();

    const backend: IBackend = new Backend();

    // Initialize session state from cookie on mount
    onMount(async () => {
        try {
            const resp = await backend.fetchJson('/api/session/me');
            if (!('ok' in resp) || !resp.ok) return;
            const data = resp.data as TSessionData | null;
            if (data && typeof (data as any).address === 'string') {
                setPageState.setSessionAddress(data.address);
                setPageState.setSessionExpiresAt(data.expires_at);
                setPageState.setSessionOk(true);
            } else {
                setPageState.setSessionAddress(undefined);
                setPageState.setSessionExpiresAt(undefined);
                setPageState.setSessionOk(false);
            }
        } catch (e) {
            console.warn('Failed to init session from cookie', e);
        }
    });

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