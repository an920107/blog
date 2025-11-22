import { toast } from 'svelte-sonner';

export interface CopyToClipboardOptions {
	successMessage?: string;
	successDescription?: string;
	errorMessage?: string;
	errorDescription?: string;
	showToast?: boolean;
}

/**
 * Copies text to clipboard and optionally shows a toast notification.
 * @param text - The text to copy to clipboard
 * @param options - Optional configuration for toast messages
 * @returns Promise<boolean> - true if copy was successful, false otherwise
 */
export async function copyToClipboard(
	text: string,
	options: CopyToClipboardOptions = {}
): Promise<boolean> {
	const {
		successMessage = 'Copied to clipboard',
		successDescription,
		errorMessage = 'Failed to copy',
		errorDescription = 'Please copy manually',
		showToast = true,
	} = options;

	try {
		await navigator.clipboard.writeText(text);

		if (showToast) {
			toast.success(successMessage, {
				description: successDescription,
			});
		}

		return true;
	} catch {
		if (showToast) {
			toast.error(errorMessage, {
				description: errorDescription,
			});
		}

		return false;
	}
}
