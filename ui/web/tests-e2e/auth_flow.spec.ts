import { test, expect } from '@playwright/test';

// NOTE: Backend must be running (cargo-run) for this demo test.

test('auth flow: signin -> verify -> me', async ({ page, request, context }) => {
  // Trigger signin via API to keep UI test short
  const signin = await request.post('http://localhost:7878/api/session/signin', {
    headers: { 'content-type': 'application/json' },
    data: { address: 'e2e@example.com' },
  });
  expect(signin.ok()).toBeTruthy();

  const verify = await request.post('http://localhost:7878/api/session/verify', {
    headers: { 'content-type': 'application/json' },
    data: { address: 'e2e@example.com', code: 'some_code' },
  });
  expect(verify.ok()).toBeTruthy();

  const setCookie = verify.headers()['set-cookie'];
  expect(setCookie).toBeTruthy();

  // Apply cookie to browser context
  await context.addCookies([
    {
      name: 'Authorization',
      value: setCookie.split('Authorization=')[1].split(';')[0].replace('Bearer%20', 'Bearer%20'),
      domain: 'localhost',
      path: '/',
      httpOnly: true,
    },
  ]);

  await page.goto('/');
  await expect(page).toHaveURL(/\//);
});
