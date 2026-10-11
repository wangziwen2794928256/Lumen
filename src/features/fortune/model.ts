// A local entertainment prompt, never a forecast or input to the planner.
export function localDay(now = new Date()): string {
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
}
export function dailyIndex(day: string, count: number): number {
  const stamp = Date.parse(`${day}T00:00:00Z`);
  if (
    !/^\d{4}-\d{2}-\d{2}$/.test(day) ||
    !Number.isFinite(stamp) ||
    new Date(stamp).toISOString().slice(0, 10) !== day ||
    !Number.isInteger(count) ||
    count < 1
  )
    throw new Error('Invalid daily sign date or collection');
  return ((Math.floor(stamp / 86400000) % count) + count) % count;
}
