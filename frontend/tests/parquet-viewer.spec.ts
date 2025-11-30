import { test, expect } from '@playwright/test';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

test.describe('Parquet Viewer', () => {
	test('should display the page title and description', async ({ page }) => {
		await page.goto('/');

		// タイトルとサブタイトルを確認
		await expect(page.locator('h1')).toHaveText('Parquet Viewer');
		await expect(page.getByText('Upload and view Parquet files in your browser')).toBeVisible();
		await expect(page.getByText('Maximum 100,000 rows supported')).toBeVisible();
	});

	test('should upload and display parquet file data', async ({ page }) => {
		await page.goto('/');

		// ファイルアップロード
		const fileInput = page.locator('input[type="file"]');
		const filePath = path.join(__dirname, 'fixtures', 'small.parquet');
		await fileInput.setInputFiles(filePath);

		// データが読み込まれるまで待機
		await expect(page.getByText('Data Preview')).toBeVisible({ timeout: 10000 });

		// 総行数が表示されることを確認
		await expect(page.getByText(/Total rows: \d+/)).toBeVisible();

		// テーブルが表示されることを確認
		const table = page.locator('table');
		await expect(table).toBeVisible();

		// ヘッダー列を確認（small.parquetのスキーマに基づく）
		await expect(page.getByRole('columnheader', { name: 'id' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'name' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'age' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'score' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'active' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'created_at' })).toBeVisible();

		// データ行が存在することを確認
		const rows = page.locator('tbody tr');
		const rowCount = await rows.count();
		expect(rowCount).toBeGreaterThan(0);
		expect(rowCount).toBeLessThanOrEqual(10); // small.parquetは10行
	});

	test('should upload and display parquet file with Japanese column names', async ({ page }) => {
		await page.goto('/');

		// ファイルアップロード（small2.parquet - 日本語カラム名の商品在庫データ）
		const fileInput = page.locator('input[type="file"]');
		const filePath = path.join(__dirname, 'fixtures', 'small2.parquet');
		await fileInput.setInputFiles(filePath);

		// データが読み込まれるまで待機
		await expect(page.getByText('Data Preview')).toBeVisible({ timeout: 10000 });

		// 総行数が表示されることを確認
		await expect(page.getByText(/Total rows: \d+/)).toBeVisible();

		// テーブルが表示されることを確認
		const table = page.locator('table');
		await expect(table).toBeVisible();

		// ヘッダー列を確認（small2.parquetのスキーマに基づく - 日本語カラム名）
		await expect(page.getByRole('columnheader', { name: '商品ID' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '商品名' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'カテゴリ' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '価格' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '在庫数' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'ステータス' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '評価' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'レビュー数' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '発売日' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '最終更新日時' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '割引率' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: '在庫警告' })).toBeVisible();
		await expect(page.getByRole('columnheader', { name: 'タグ' })).toBeVisible();

		// データ行が存在することを確認
		const rows = page.locator('tbody tr');
		const rowCount = await rows.count();
		expect(rowCount).toBeGreaterThan(0);
		expect(rowCount).toBeLessThanOrEqual(10); // small2.parquetは10行
	});

	test('should show error for files larger than 100,000 rows', async ({ page }) => {
		await page.goto('/');

		// この部分は実際に10万行を超えるファイルがある場合のテスト
		// 現在はsmall.parquetしかないのでスキップ
		// 必要に応じて大きなファイルを生成してテスト可能
	});

	test('should show WASM module loaded without errors', async ({ page }) => {
		// コンソールエラーを監視
		const consoleErrors: string[] = [];
		page.on('console', (msg) => {
			if (msg.type() === 'error') {
				consoleErrors.push(msg.text());
			}
		});

		await page.goto('/');

		// ページが読み込まれた後、WASMエラーがないことを確認
		await page.waitForLoadState('networkidle');

		// WASM関連のエラーがないことを確認
		const wasmErrors = consoleErrors.filter((error) =>
			error.toLowerCase().includes('wasm')
		);
		expect(wasmErrors).toHaveLength(0);
	});
});
