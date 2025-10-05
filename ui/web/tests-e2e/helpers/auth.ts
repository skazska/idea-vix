import type { APIRequestContext, BrowserContext } from '@playwright/test';

/**
 * Signs in via backend API and applies the HttpOnly Authorization cookie to the browser context.
 */
export async function apiSignInAndApplyCookie(
  request: APIRequestContext,
  context: BrowserContext,
  address = 'e2e@example.com',
  code = 'some_code'
): Promise<void> {
  const signin = await request.post(`http://localhost:${process.env.WS_PORT}/api/session/signin`, {
    headers: { 'content-type': 'application/json' },
    data: { address },
  });
  if (!signin.ok()) throw new Error('signin failed');

  const verify = await request.post(`http://localhost:${process.env.WS_PORT}/api/session/verify`, {
    headers: { 'content-type': 'application/json' },
    data: { address, code },
  });
  if (!verify.ok()) throw new Error('verify failed');

  const setCookie = verify.headers()['set-cookie'];
  if (!setCookie) throw new Error('no set-cookie header');

  const value = setCookie.split('Authorization=')[1]?.split(';')[0];
  if (!value) throw new Error('cookie parse failed');

  await context.addCookies([
    {
      name: 'Authorization',
      value,
      domain: 'localhost',
      path: '/',
      httpOnly: true,
    },
  ]);
}
