import { test, expect, type Page } from '@playwright/test';
import { apiSignInAndApplyCookie } from './helpers/auth';

async function createPackage(page: Page, name: string, isPublic = false) {
  await page.goto('/package');
  await page.getByTestId('package-add-button').click();
  const form = page.getByTestId('package-new-form');
  await expect(form).toBeVisible();
  await form.locator('input[name="name"]').fill(name);
  if (isPublic) {
    const cb = form.locator('input[name="is_public"]');
    const checked = await cb.isChecked();
    if (!checked) await cb.click();
  }
  await form.locator('button[type="submit"]').click();
  await expect(page.getByTestId('modal-centered')).toBeHidden();
  const row = page.getByTestId('package-items').getByRole('listitem').filter({ hasText: name });
  await expect(row).toBeVisible();
  await row.getByTestId('open-package-item').click();
}

test.describe('Package access management', () => {
  test.beforeEach(async ({ request, context }) => {
    await apiSignInAndApplyCookie(request, context, 'owner@example.com', 'some_code');
  });

  test('grant and revoke access for package', async ({ page }) => {
    const name = `pkg-access-${Date.now()}`;
    await createPackage(page, name, true);

    const form = page.getByTestId('package-access-form');
    await expect(form).toBeVisible();

    const address = `user-${Date.now()}@example.com`;

    await form.locator('input[name="address"]').fill(address);
    await form.locator('select[name="role"]').selectOption('manage');
    await page.getByTestId('package-access-grant-button').click();

    const list = page.getByTestId('package-access-list');
    await expect(list).toBeVisible();
    const row = list.getByRole('listitem').filter({ hasText: address });
    await expect(row).toBeVisible();
    await expect(row.getByTestId('package-access-role')).toHaveText(/manage/i);

    // revoke
    await row.getByTestId('package-access-revoke-button').click();
    await expect(list.getByRole('listitem').filter({ hasText: address })).toHaveCount(0);
  });
});
