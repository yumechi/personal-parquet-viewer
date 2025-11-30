<script lang="ts">
	import { onMount } from 'svelte';
	import type { ParquetData } from '$lib/types';

	let fileInput: HTMLInputElement;
	let parquetData: ParquetData | null = null;
	let loading = false;
	let error: string | null = null;
	let wasmModule: any = null;

	onMount(async () => {
		try {
			const wasm = await import('parquet-viewer-wasm');
			await wasm.default();
			wasm.init_panic_hook();
			wasmModule = wasm;
		} catch (e) {
			error = `Failed to load WASM module: ${e}`;
			console.error('WASM loading error:', e);
		}
	});

	async function handleFileUpload(event: Event) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];

		if (!file) return;

		loading = true;
		error = null;
		parquetData = null;

		try {
			const arrayBuffer = await file.arrayBuffer();
			const uint8Array = new Uint8Array(arrayBuffer);

			const result = wasmModule.read_parquet(uint8Array);
			parquetData = result;
		} catch (e) {
			error = `Error reading Parquet file: ${e}`;
		} finally {
			loading = false;
		}
	}
</script>

<div class="min-h-screen bg-gray-50 py-8">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="text-center mb-8">
			<h1 class="text-4xl font-bold text-gray-900 mb-2">Parquet Viewer</h1>
			<p class="text-gray-600">Upload and view Parquet files in your browser</p>
		</div>

		<div class="bg-white rounded-lg shadow-md p-6 mb-6">
			<div class="flex items-center justify-center w-full">
				<label
					class="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 hover:bg-gray-100"
				>
					<div class="flex flex-col items-center justify-center pt-5 pb-6">
						<svg
							class="w-10 h-10 mb-3 text-gray-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
							/>
						</svg>
						<p class="mb-2 text-sm text-gray-500">
							<span class="font-semibold">Click to upload</span> or drag and drop
						</p>
						<p class="text-xs text-gray-500">Parquet files only</p>
					</div>
					<input
						bind:this={fileInput}
						type="file"
						class="hidden"
						accept=".parquet"
						on:change={handleFileUpload}
					/>
				</label>
			</div>
		</div>

		{#if loading}
			<div class="bg-white rounded-lg shadow-md p-6 text-center">
				<div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
				<p class="mt-2 text-gray-600">Loading Parquet file...</p>
			</div>
		{/if}

		{#if error}
			<div class="bg-red-50 border border-red-200 rounded-lg p-4 mb-6">
				<p class="text-red-800">{error}</p>
			</div>
		{/if}

		{#if parquetData}
			<div class="bg-white rounded-lg shadow-md p-6">
				<div class="mb-4">
					<h2 class="text-2xl font-bold text-gray-900">Data Preview</h2>
					<p class="text-gray-600">Total rows: {parquetData.total_rows}</p>
				</div>

				<div class="overflow-x-auto">
					<table class="min-w-full divide-y divide-gray-200">
						<thead class="bg-gray-50">
							<tr>
								{#each parquetData.columns as column}
									<th
										class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
									>
										{column}
									</th>
								{/each}
							</tr>
						</thead>
						<tbody class="bg-white divide-y divide-gray-200">
							{#each parquetData.rows.slice(0, 100) as row}
								<tr>
									{#each row as cell}
										<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
											{cell}
										</td>
									{/each}
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				{#if parquetData.total_rows > 100}
					<div class="mt-4 text-center text-gray-600">
						<p>Showing first 100 rows of {parquetData.total_rows}</p>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
