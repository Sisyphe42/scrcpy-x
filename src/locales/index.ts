import { createI18n } from 'vue-i18n';
import en from './en';
import zh from './zh';

// Get saved language preference or detect from browser
function getDefaultLocale(): string {
  // Check localStorage first
  if (typeof localStorage !== 'undefined') {
    const saved = localStorage.getItem('scrcpyx-locale');
    if (saved && (saved === 'en' || saved === 'zh')) {
      return saved;
    }
  }
  
  // Detect from browser
  if (typeof navigator !== 'undefined') {
    const browserLang = navigator.language.toLowerCase();
    if (browserLang.startsWith('zh')) {
      return 'zh';
    }
  }
  
  return 'en';
}

export const i18n = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
  fallbackLocale: 'en',
  messages: {
    en,
    zh,
  },
});

// Helper to change language
export function setLocale(locale: string): void {
  if (locale === 'en' || locale === 'zh') {
    i18n.global.locale.value = locale as 'en' | 'zh';
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('scrcpyx-locale', locale);
    }
  }
}

// Helper to get current language
export function getLocale(): string {
  return i18n.global.locale.value;
}

// Available languages
export const availableLocales = [
  { label: 'English', value: 'en' },
  { label: '中文', value: 'zh' },
];

export default i18n;
