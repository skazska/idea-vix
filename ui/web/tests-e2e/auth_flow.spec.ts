import { test, expect } from '@playwright/test';
import { apiSignInAndApplyCookie } from './helpers/auth';

// NOTE: Backend must be running (cargo-run) for this demo test.

// test('auth flow: signin -> verify -> me', async ({ page, request, context }) => {
//   // Trigger signin via API to keep UI test short
//   const signin = await request.post('http://localhost:7878/api/session/signin', {
//     headers: { 'content-type': 'application/json' },
//     data: { address: 'e2e@example.com' },
//   });
//   expect(signin.ok()).toBeTruthy();

//   const verify = await request.post('http://localhost:7878/api/session/verify', {
//     headers: { 'content-type': 'application/json' },
//     data: { address: 'e2e@example.com', code: 'some_code' },
//   });
//   expect(verify.ok()).toBeTruthy();

//   const setCookie = verify.headers()['set-cookie'];
//   expect(setCookie).toBeTruthy();

//   // Apply cookie to browser context
//   await context.addCookies([
//     {
//       name: 'Authorization',
//       value: setCookie.split('Authorization=')[1].split(';')[0].replace('Bearer%20', 'Bearer%20'),
//       domain: 'localhost',
//       path: '/',
//       httpOnly: true,
//     },
//   ]);

//   await page.goto('/');
//   await expect(page).toHaveURL(/\//);
// });

test('auth flow via UI: open modal, signin, verify, user is signed in', async ({ page }) => {
  await page.goto('/');

  let sessionIcon = page.locator('#session-icon');
  await expect(sessionIcon).toBeVisible();

  // Open session modal via the user icon
  await sessionIcon.click();
  const modal = page.locator('#new-package-item-modal');
  await expect(modal).toBeVisible();
  await expect(page.locator('#session-signin-form')).toBeVisible();

  // Fill email and submit
  await page.locator('input[name="address"]').fill('e2e@example.com');
  await page.locator('button[type="submit"]').click();

  // Verify code step
  await expect(page.locator('#session-verify-form')).toBeVisible();
  await page.locator('input[name="code"]').fill('some_code');
  await page.locator('button[type="submit"]').click();

  // Modal closes and icon shows signed-in state
  await expect(modal).toBeHidden();
  await expect(sessionIcon).toHaveAttribute('title', 'Signed in as e2e@example.com');

  page.reload();
  sessionIcon = page.locator('#session-icon');
  await expect(sessionIcon).toHaveAttribute('title', 'Signed in as e2e@example.com');

  // Optional: open session info and verify it shows
  await sessionIcon.click();
  await expect(page.locator('#session-info-modal')).toBeVisible();
});

test('auth flow via UI: sign out clears session', async ({ page, context, request }) => {
  await page.goto('/');

  // Pre-authenticate via API to focus this test on the logout UI only
  await apiSignInAndApplyCookie(request, context, 'e2e@example.com', 'some_code');
  await page.reload();

  const sessionIcon = page.locator('#session-icon');
  await expect(sessionIcon).toHaveAttribute('title', 'Signed in as e2e@example.com');

  const modal = page.locator('#new-package-item-modal');

  // Cookie should be present before signout
  const cookiesBefore = await context.cookies();
  expect(cookiesBefore.some(c => c.name === 'Authorization')).toBeTruthy();

  // Open session info and sign out
  await sessionIcon.click();
  await expect(page.locator('#session-info-modal')).toBeVisible();
  await page.locator('#session-signout-button').click();

  // Modal closes and icon shows signed-out state
  await expect(modal).toBeHidden();
  await expect(sessionIcon).toHaveAttribute('title', 'Sign in');

  // Cookie should be cleared after signout
  const cookiesAfter = await context.cookies();
  expect(cookiesAfter.some(c => c.name === 'Authorization')).toBeFalsy();
});
