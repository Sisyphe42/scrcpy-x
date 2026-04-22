import { useMessage } from 'naive-ui';

// Simple wrappers around Naive UI message API
const message = useMessage();

export function showSuccess(msg: string) {
  message.success(msg);
}

export function showError(msg: string) {
  message.error(msg);
}

export function showWarning(msg: string) {
  message.warning(msg);
}

export function showInfo(msg: string) {
  message.info(msg);
}
