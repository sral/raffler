/**
 * Format a duration in minutes to a human-readable string
 * @param {number} minutes - Duration in minutes
 * @returns {string} Formatted duration string
 */
export function formatDuration(minutes) {
  const days = Math.floor(minutes / (24 * 60));
  const hours = Math.floor((minutes % (24 * 60)) / 60);
  const mins = minutes % 60;

  const parts = [];
  if (days > 0) parts.push(`${days} day${days === 1 ? '' : 's'}`);
  if (hours > 0) parts.push(`${hours} hour${hours === 1 ? '' : 's'}`);
  if (mins > 0) parts.push(`${mins} minute${mins === 1 ? '' : 's'}`);

  if (parts.length === 0) return '0 minutes';
  if (parts.length === 1) return parts[0];
  if (parts.length === 2) return `${parts[0]} and ${parts[1]}`;
  return `${parts[0]}, ${parts[1]} and ${parts[2]}`;
}

/**
 * Format a timestamp to a date and time string
 * @param {string} timestamp - ISO timestamp
 * @returns {string} Formatted date time string
 */
export function formatDateTime(timestamp) {
  const [date, time] = timestamp.replace('T', ' ').split(' ');
  const [hour, minute] = time.split(':');
  return `${date} ${hour}:${minute}`;
}
