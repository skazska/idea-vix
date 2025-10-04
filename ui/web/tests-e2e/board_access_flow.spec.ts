import { test, expect, type Page } from '@playwright/test';
import { apiSignInAndApplyCookie } from './helpers/auth';

async function createBoard(page: Page, name: string, isPublic = false) {
  await page.goto('/board');
  await page.getByTestId('board-add-button').click();
  const form = page.getByTestId('board-new-form');
  await expect(form).toBeVisible();
  await form.locator('input[name="name"]').fill(name);
  if (isPublic) {
    const cb = form.locator('input[name="is_public"]');
    const checked = await cb.isChecked();
    if (!checked) await cb.click();
  }
  await form.locator('button[type="submit"]').click();
  await expect(page.getByTestId('modal-centered')).toBeHidden();
  const row = page.getByRole('listitem').filter({ hasText: name });
  await expect(row).toBeVisible();
  await row.getByRole('link').first().click();
}

test.describe('@Smmoke Board access management', () => {
  test.beforeEach(async ({ request, context }) => {
    await apiSignInAndApplyCookie(request, context, 'owner@example.com', 'some_code');
  });

  test('grant and revoke access for board', async ({ page }) => {
    const name = `brd-access-${Date.now()}`;
    await createBoard(page, name, true);

  // Open the Access section (expandable defaults to closed now)
  await page.getByTestId('board-section-access-toggle').click();
  await expect(page.getByTestId('board-section-access-content')).toBeVisible();

    const form = page.getByTestId('access-map-form');
    await expect(form).toBeVisible();

    const address = `user-${Date.now()}@example.com`;

    await form.locator('input[name="address"]').fill(address);
    await form.locator('select[name="role"]').selectOption('edit');
    await page.getByTestId('access-map-grant-button').click();

    const list = page.getByTestId('access-map-list');
    await expect(list).toBeVisible();
    const row = list.getByRole('listitem').filter({ hasText: address });
    await expect(row).toBeVisible();
    await expect(row.getByTestId('access-map-role')).toHaveText(/edit/i);

    // revoke
    await row.getByTestId('access-map-revoke-button').click();
    await expect(list.getByRole('listitem').filter({ hasText: address })).toHaveCount(0);
  });
});
