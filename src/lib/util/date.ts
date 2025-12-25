const intervals: Record<number, Intl.RelativeTimeFormatUnit> = {
  525600: 'year',
  43200: 'month',
  10080: 'week',
  1440: 'day',
  60: 'hour',
  1: 'minute',
}

export function getRelativeTime(date: number, rtf: Intl.RelativeTimeFormat): string {
  const minutes = ((new Date()).getTime() - date) / (1000 * 60);
  for (const interval of Object.keys(intervals).reverse()) {
    const units = minutes / +interval;
    if (units >= 1) return rtf.format(-Math.round(units), intervals[+interval]);
  }
  return '';
}