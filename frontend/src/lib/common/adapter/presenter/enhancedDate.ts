export class EnhancedDate extends Date {
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
		const pad = (n: number, z = 2) => String(n).padStart(z, '0');
		const year = this.getFullYear();
		const month = pad(this.getMonth() + 1);
		const day = pad(this.getDate());
		return `${year}-${month}-${day}`;
	}

	toLocalISOTime(): string {
		const pad = (n: number, z = 2) => String(n).padStart(z, '0');
		const hours = pad(this.getHours());
		const minutes = pad(this.getMinutes());
		const seconds = pad(this.getSeconds());
		const ms = this.getMilliseconds();
		const msStr = ms ? '.' + String(ms).padStart(3, '0') : '';
		return `${hours}:${minutes}:${seconds}${msStr}`;
	}

	toLocalDateTimeInputValue(): string {
		const pad = (n: number, z = 2) => String(n).padStart(z, '0');
		const year = this.getFullYear();
		const month = pad(this.getMonth() + 1);
		const day = pad(this.getDate());
		const hours = pad(this.getHours());
		const minutes = pad(this.getMinutes());
		return `${year}-${month}-${day}T${hours}:${minutes}`;
	}
}
