import type { Color } from '$lib/label/domain/entity/color';

export class ColorViewModel {
	readonly red: number;
	readonly green: number;
	readonly blue: number;
	readonly alpha: number;

	private constructor(props: { red: number; green: number; blue: number; alpha: number }) {
		this.red = props.red;
		this.green = props.green;
		this.blue = props.blue;
		this.alpha = props.alpha;
	}

	private static fromHsl(hsl: Hsl): ColorViewModel {
		const { h, s, l } = hsl;
		let r, g, b;

		if (s === 0) {
			// achromatic (grayscale)
			r = g = b = l;
		} else {
			const hue2rgb = (p: number, q: number, t: number) => {
				if (t < 0) t += 1;
				if (t > 1) t -= 1;
				if (t < 1 / 6) return p + (q - p) * 6 * t;
				if (t < 1 / 2) return q;
				if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
				return p;
			};

			const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
			const p = 2 * l - q;
			const h_norm = h / 360;

			r = hue2rgb(p, q, h_norm + 1 / 3);
			g = hue2rgb(p, q, h_norm);
			b = hue2rgb(p, q, h_norm - 1 / 3);
		}

		return new ColorViewModel({
			red: Math.round(r * 255),
			green: Math.round(g * 255),
			blue: Math.round(b * 255),
			alpha: 255,
		});
	}

	static fromEntity(color: Color): ColorViewModel {
		return new ColorViewModel({
			red: color.red,
			green: color.green,
			blue: color.blue,
			alpha: color.alpha,
		});
	}

	static rehydrate(props: DehydratedColorProps): ColorViewModel {
		return new ColorViewModel(props);
	}

	get hex(): string {
		const toHex = (value: number) => value.toString(16).padStart(2, '0');
		return `#${toHex(this.red)}${toHex(this.green)}${toHex(this.blue)}${toHex(this.alpha)}`;
	}

	private toHsl(): Hsl {
		const r = this.red / 255;
		const g = this.green / 255;
		const b = this.blue / 255;

		const max = Math.max(r, g, b);
		const min = Math.min(r, g, b);
		let h = 0,
			s = 0;
		const l = (max + min) / 2;

		if (max === min) {
			// achromatic (grayscale)
			h = s = 0;
		} else {
			const d = max - min;
			s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
			switch (max) {
				case r:
					h = (g - b) / d + (g < b ? 6 : 0);
					break;
				case g:
					h = (b - r) / d + 2;
					break;
				case b:
					h = (r - g) / d + 4;
					break;
			}
			h /= 6;
		}

		return { h: h * 360, s: s, l: l };
	}

	lighten(amount: number): ColorViewModel {
		const hsl = this.toHsl();
		hsl.l += amount;
		hsl.l = Math.max(0, Math.min(1, hsl.l));
		return ColorViewModel.fromHsl(hsl);
	}

	darken(amount: number): ColorViewModel {
		return this.lighten(-amount);
	}

	dehydrate(): DehydratedColorProps {
		return {
			red: this.red,
			green: this.green,
			blue: this.blue,
			alpha: this.alpha,
		};
	}
}

interface Hsl {
	h: number;
	s: number;
	l: number;
}

export interface DehydratedColorProps {
	red: number;
	green: number;
	blue: number;
	alpha: number;
}
