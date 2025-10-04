import { test, expect, type Locator } from '@playwright/test';
import { apiSignInAndApplyCookie } from './helpers/auth';

type SessionLocators = {
  sessionIcon: Locator;
  modal: Locator;
  signinForm: Locator;
  verifyForm: Locator;
  sessionInfo: Locator;
  signoutButton: Locator;
};

test.describe('@Smoke Authentication Flow', () => {
  let locators: SessionLocators;

  test.beforeEach(async ({ page }) => {
    await page.goto('/');

    locators = {
      sessionIcon: page.getByTestId('session-icon'),
      modal: page.getByTestId('modal-centered'),
      signinForm: page.getByTestId('session-signin-form'),
      verifyForm: page.getByTestId('session-verify-form'),
      sessionInfo: page.getByTestId('session-info-modal'),
      signoutButton: page.getByTestId('session-signout-button'),
    };
  });

  test('auth flow via UI: open modal, signin, verify, user is signed in', async ({ page }) => {
   const { sessionIcon, modal, signinForm, verifyForm, sessionInfo } = locators;

    await expect(modal).toBeHidden();
    await expect(signinForm).toBeHidden();
    await expect(verifyForm).toBeHidden();

    // Ensure session icon is present and not signed in
    await expect(sessionIcon).toBeVisible();
    await expect(sessionIcon).toHaveAttribute('title', 'Sign in');

    // Open session modal via the user icon
    await sessionIcon.click();
    await expect(signinForm).toBeVisible();
    await expect(modal).toBeVisible();

    // Fill email and submit
    await signinForm.locator('input[name="address"]').fill('e2e@example.com');
    await signinForm.locator('button[type="submit"]').click();

    await expect(signinForm).toBeHidden();
    
    // Verify code step
    await expect(verifyForm).toBeVisible();
    await verifyForm.locator('input[name="code"]').fill('some_code');
    await verifyForm.locator('button[type="submit"]').click();

    await expect(verifyForm).toBeHidden();
    await expect(modal).toBeHidden();
    await expect(sessionIcon).toHaveAttribute('title', 'Signed in as e2e@example.com');

    page.reload();
    await expect(sessionIcon).toHaveAttribute('title', 'Signed in as e2e@example.com');

    // Optional: open session info and verify it shows
    await sessionIcon.click();
    await expect(sessionInfo).toBeVisible();
  });

  test('auth flow via UI: sign out clears session', async ({ page, context, request }) => {
    const { sessionIcon, modal, sessionInfo, signoutButton } = locators;
    // Pre-authenticate via API to focus this test on the logout UI only
    await apiSignInAndApplyCookie(request, context, 'e2e@example.com', 'some_code');
    await page.reload();

    await expect(sessionIcon).toHaveAttribute('title', 'Signed in as e2e@example.com');

    await expect(modal).toBeHidden();

    // Cookie should be present before signout
    const cookiesBefore = await context.cookies();
    expect(cookiesBefore.some(c => c.name === 'Authorization')).toBeTruthy();

    // Open session info and sign out
    await sessionIcon.click();
    await expect(sessionInfo).toBeVisible();
    await signoutButton.click();

    // Modal closes and icon shows signed-out state
    await expect(modal).toBeHidden();
    await expect(sessionIcon).toHaveAttribute('title', 'Sign in');

    // Cookie should be cleared after signout
    const cookiesAfter = await context.cookies();
    expect(cookiesAfter.some(c => c.name === 'Authorization')).toBeFalsy();
  });
});