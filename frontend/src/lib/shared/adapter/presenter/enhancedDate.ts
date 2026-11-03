export class EnhancedDate {
	// Using composition over inheritance to avoid issues with hydration in SvelteKit
	private date: Date;

	constructor(date?: Date | number | string) {
		if (date === undefined) {
			this.date = new Date();
			return;
		}
		this.date = new Date(date);
	}

	get nativeDate(): Date {
		return this.date;
	}

	getFullYear(): number {
		return this.date.getFullYear();
	}

	getMonth(): number {
		return this.date.getMonth();
	}

	getDate(): number {
		return this.date.getDate();
	}

	getHours(): number {
		return this.date.getHours();
	}

	getMinutes(): number {
		return this.date.getMinutes();
	}

	getSeconds(): number {
		return this.date.getSeconds();
	}

	getMilliseconds(): number {
		return this.date.getMilliseconds();
	}

	getTimezoneOffset(): number {
		return this.date.getTimezoneOffset();
	}

	getTime(): number {
		return this.date.getTime();
	}

	toISOString(): string {
		return this.date.toISOString();
	}

	toLocalISOString(): string {
		const pad = (n: number, z = 2) => String(n).padStart(z, '0');
		const year = this.getFullYear();
		const month = pad(this.getMonth() + 1);
		const day = pad(this.getDate());
		const hours = pad(this.getHours());
		const minutes = pad(this.getMinutes());
		const seconds = pad(this.getSeconds());
		const ms = this.getMilliseconds();
		const msStr = ms ? '.' + String(ms).padStart(3, '0') : '';

		// timezone offset in minutes. JavaScript returns minutes behind UTC, so invert sign
		const offsetMinutesTotal = -this.getTimezoneOffset();
		const sign = offsetMinutesTotal >= 0 ? '+' : '-';
		const absOffset = Math.abs(offsetMinutesTotal);
		const offsetH = pad(Math.floor(absOffset / 60));
		const offsetM = pad(absOffset % 60);

		return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}${msStr}${sign}${offsetH}:${offsetM}`;
	}

	toLocalISODate(): string {
		return this.toLocalISOString().slice(0, 10);
	}

	toLocalISOTime(): string {
		return this.toLocalISOString().slice(11, 19);
	}

	toDateTimeInputValue(): string {
		return this.toLocalISOString().slice(0, 16);
	}
}
