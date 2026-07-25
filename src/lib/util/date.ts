const intervals: Array<[number, Intl.RelativeTimeFormatUnit]> = [
	[525600, 'year'],
	[43200, 'month'],
	[10080, 'week'],
	[1440, 'day'],
	[60, 'hour'],
	[1, 'minute'],
];

export function getRelativeTime(
	date: number,
	rtf: Intl.RelativeTimeFormat,
	now: number,
): string {
	const minutes = (now - date * 1000) / (1000 * 60);

	for (const [threshold, unit] of intervals) {
		const units = minutes / threshold;
		if (units >= 1) return rtf.format(-Math.round(units), unit);
	}
	return rtf.format(-0, 'minute');
}
