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

async function createPackage(page: Page, name: string, slug: string, isPublic = false) {
  await page.goto('/package');
  await page.getByTestId('package-add-button').click();
  const form = page.getByTestId('package-new-form');
  await expect(form).toBeVisible();
  await form.locator('input[name="name"]').fill(name);
  await form.locator('input[name="slug"]').fill(slug);
  if (isPublic) {
    const cb = form.locator('input[name="is_public"]');
    const checked = await cb.isChecked();
    if (!checked) await cb.click();
  }
  await form.locator('button[type="submit"]').click();
  await expect(page.getByTestId('modal-centered')).toBeHidden();
}

test.describe('@smoke Board package management', () => {
  test.beforeEach(async ({ request, context }) => {
    await apiSignInAndApplyCookie(request, context, 'owner@example.com', 'some_code');
  });

  test('owner can add and remove packages from board', async ({ page }) => {
    const timestamp = Date.now();
    const boardName = `brd-pkg-${timestamp}`;
    const pkg1Slug = `pkg1-${timestamp}`;
    const pkg2Slug = `pkg2-${timestamp}`;

    // Create test packages
    await createPackage(page, `Package 1`, pkg1Slug, true);
    await createPackage(page, `Package 2`, pkg2Slug, true);

    // Create board and navigate to it
    await createBoard(page, boardName, true);

    // Open the Packages section
    await page.getByTestId('board-section-packages-toggle').click();
    await expect(page.getByTestId('board-section-packages-content')).toBeVisible();

    // Verify empty state
    await expect(page.getByText(/No packages added yet/i)).toBeVisible();

    // Click Add Package button
    await page.getByTestId('add-package-button').click();
    
    // Modal should be visible
    const modal = page.getByTestId('modal-centered');
    await expect(modal).toBeVisible();

    // Search for first package
    await page.getByTestId('package-search-input').fill(pkg1Slug);
    
    // Wait for package to appear in list
    await expect(page.getByTestId(`available-package-${pkg1Slug}`)).toBeVisible();
    
    // Add first package
    await page.getByTestId(`add-package-to-board-${pkg1Slug}`).click();
    
    // Modal should close
    await expect(modal).toBeHidden();

    // Wait a moment for the refetch to complete and UI to update
    await page.waitForTimeout(500);

    // Verify package appears in board list
    await expect(page.getByTestId(`board-package-${pkg1Slug}`)).toBeVisible();

    // Add second package
    await page.getByTestId('add-package-button').click();
    await expect(modal).toBeVisible();
    await page.getByTestId('package-search-input').fill(pkg2Slug);
    await expect(page.getByTestId(`available-package-${pkg2Slug}`)).toBeVisible();
    await page.getByTestId(`add-package-to-board-${pkg2Slug}`).click();
    await expect(modal).toBeHidden();

    // Verify both packages are in the list
    await expect(page.getByTestId(`board-package-${pkg1Slug}`)).toBeVisible();
    await expect(page.getByTestId(`board-package-${pkg2Slug}`)).toBeVisible();

    // Remove first package
    page.once('dialog', dialog => dialog.accept());
    await page.getByTestId(`remove-package-${pkg1Slug}`).click();
    
    // Verify first package removed, second remains
    await expect(page.getByTestId(`board-package-${pkg1Slug}`)).toHaveCount(0);
    await expect(page.getByTestId(`board-package-${pkg2Slug}`)).toBeVisible();

    // Remove second package
    page.once('dialog', dialog => dialog.accept());
    await page.getByTestId(`remove-package-${pkg2Slug}`).click();
    
    // Verify empty state appears again
    await expect(page.getByText(/No packages added yet/i)).toBeVisible();
  });

  test('added packages are filtered from available list', async ({ page }) => {
    const timestamp = Date.now();
    const boardName = `brd-filter-${timestamp}`;
    const pkgSlug = `pkg-filter-${timestamp}`;

    // Create test package
    await createPackage(page, `Filter Test Package`, pkgSlug, true);

    // Create board
    await createBoard(page, boardName, true);

    // Open Packages section
    await page.getByTestId('board-section-packages-toggle').click();
    await expect(page.getByTestId('board-section-packages-content')).toBeVisible();

    // Add package
    await page.getByTestId('add-package-button').click();
    await expect(page.getByTestId('modal-centered')).toBeVisible();
    await page.getByTestId('package-search-input').fill(pkgSlug);
    await expect(page.getByTestId(`available-package-${pkgSlug}`)).toBeVisible();
    await page.getByTestId(`add-package-to-board-${pkgSlug}`).click();
    await expect(page.getByTestId('modal-centered')).toBeHidden();

    // Wait for refetch to complete
    await page.waitForTimeout(500);

    // Open modal again
    await page.getByTestId('add-package-button').click();
    await expect(page.getByTestId('modal-centered')).toBeVisible();
    
    // Search for the same package
    await page.getByTestId('package-search-input').fill(pkgSlug);
    
    // Package should NOT appear in available list (already added)
    await expect(page.getByTestId(`available-package-${pkgSlug}`)).toHaveCount(0);
    
    // Should show "all packages added" message or no results
    const noResultsText = page.getByText(/All available packages have been added|No packages found/i);
    await expect(noResultsText).toBeVisible();

    // Close modal
    await page.getByTestId('close-modal-button').click();
  });

  test('package search filters available packages', async ({ page }) => {
    const timestamp = Date.now();
    const boardName = `brd-search-${timestamp}`;
    const pkg1Slug = `alpha-${timestamp}`;
    const pkg2Slug = `beta-${timestamp}`;

    // Create test packages with different names
    await createPackage(page, `Alpha Package`, pkg1Slug, true);
    await createPackage(page, `Beta Package`, pkg2Slug, true);

    // Create board
    await createBoard(page, boardName, true);

    // Open Packages section
    await page.getByTestId('board-section-packages-toggle').click();
    await expect(page.getByTestId('board-section-packages-content')).toBeVisible();

    // Open add modal
    await page.getByTestId('add-package-button').click();
    await expect(page.getByTestId('modal-centered')).toBeVisible();

    // Search for "alpha"
    await page.getByTestId('package-search-input').fill('alpha');
    
    // Only alpha package should be visible
    await expect(page.getByTestId(`available-package-${pkg1Slug}`)).toBeVisible();
    await expect(page.getByTestId(`available-package-${pkg2Slug}`)).toHaveCount(0);

    // Search for "beta"
    await page.getByTestId('package-search-input').fill('beta');
    
    // Only beta package should be visible
    await expect(page.getByTestId(`available-package-${pkg1Slug}`)).toHaveCount(0);
    await expect(page.getByTestId(`available-package-${pkg2Slug}`)).toBeVisible();

    // Clear search
    await page.getByTestId('package-search-input').fill('');
    
    // Both packages should be visible
    await expect(page.getByTestId(`available-package-${pkg1Slug}`)).toBeVisible();
    await expect(page.getByTestId(`available-package-${pkg2Slug}`)).toBeVisible();

    // Close modal
    await page.getByTestId('close-modal-button').click();
  });

  test('packages section only visible to owner', async ({ page, request, context }) => {
    const timestamp = Date.now();
    const boardName = `brd-owner-only-${timestamp}`;

    // Create board as owner
    await createBoard(page, boardName, true);

    // Verify Packages section is visible to owner
    await expect(page.getByTestId('board-section-packages-toggle')).toBeVisible();

    // Sign in as different user (non-owner)
    await apiSignInAndApplyCookie(request, context, 'viewer@example.com', 'some_code');
    
    // Navigate to the same board
    await page.reload();

    // Packages section should NOT be visible to non-owner
    await expect(page.getByTestId('board-section-packages-toggle')).toHaveCount(0);
  });
});
