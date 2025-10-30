import dayjs from 'dayjs';

export function formatTime(time, format = 'YYYY-MM-DD HH:mm:ss') {
  return dayjs(Number(time) * 1000).format(format);
}
