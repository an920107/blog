/** @type {import('tailwindcss').Config} */
module.exports = {
	theme: {
		extend: {
			typography: () => ({
				gray: {
					css: {
						'--tw-prose-body': 'var(--color-gray-700)',
						'--tw-prose-headings': 'var(--color-gray-800)',
						'--tw-prose-links': 'var(--color-gray-800)',
						'--tw-prose-bold': 'var(--color-gray-800)',
						'--tw-prose-quotes': 'var(--color-gray-800)',
					},
				},
			}),
		},
	},
};
