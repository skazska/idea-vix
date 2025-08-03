/* @refresh reload */

import { query } from "@solidjs/router";
import { getResponse, type IBackend } from "../../common/providers/backend";
import type { TSignInRequest, TVerifyCodeRequest, TSessionData } from "../model";

export async function signIn(backend: IBackend, request: TSignInRequest): Promise<boolean> {
    return getResponse(backend.fetchJson('/api/session/signin', {
        method: 'POST',
        body: JSON.stringify(request),
        headers: { 'Content-Type': 'application/json' }
    }), (_data) => true, '/api/session/signin');
}

export async function verifyCode(backend: IBackend, request: TVerifyCodeRequest): Promise<TSessionData> {
    return getResponse(backend.fetchJson('/api/session/verify', {
        method: 'POST',
        body: JSON.stringify(request),
        headers: { 'Content-Type': 'application/json' }
    }), (data) => data as TSessionData, '/api/session/verify');
}

export async function signOut(backend: IBackend): Promise<boolean> {
    return getResponse(backend.fetchJson('/api/session/signout', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
    }), (_data) => true, '/api/session/signout');
}

export class SessionApi {
    private _backend: IBackend

    constructor(backend: IBackend) {
        this._backend = backend;
    }

    public get backend() {
        return this._backend;
    }

    public signIn = query((request: TSignInRequest) => signIn(this.backend, request), "signIn")
    public verifyCode = query((request: TVerifyCodeRequest) => verifyCode(this.backend, request), "verifyCode")
    public signOut = query(() => signOut(this.backend), "signOut")
}

let sessionApi: SessionApi | undefined;

export function getSessionApi(backend: IBackend): SessionApi {
    if (!sessionApi || sessionApi.backend !== backend) {
        sessionApi = new SessionApi(backend);
    }
    return sessionApi;
}
