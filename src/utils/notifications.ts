// Simple wrappers around Naive UI message API
// Note: These functions should only be called within component setup context
// For use outside components, pass the message instance

let messageInstance: ReturnType<typeof import('naive-ui').useMessage> | null = null;

export function setMessageInstance(instance: ReturnType<typeof import('naive-ui').useMessage>) {
  messageInstance = instance;
}

export function showSuccess(msg: string) {
  messageInstance?.success(msg);
}

export function showError(msg: string) {
  messageInstance?.error(msg);
}

export function showWarning(msg: string) {
  messageInstance?.warning(msg);
}

export function showInfo(msg: string) {
  messageInstance?.info(msg);
}
