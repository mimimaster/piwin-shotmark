/** Display language for the capture overlay and the pin window. */
export type DesktopLocale = 'zh-CN' | 'en';

const LOCALE_KEY = 'piwin-shotmark.locale';
const DEFAULT_LOCALE: DesktopLocale = 'zh-CN';

export function loadDesktopLocale(): DesktopLocale {
  try {
    const stored = localStorage.getItem(LOCALE_KEY);
    if (stored === 'zh-CN' || stored === 'en') {
      return stored;
    }
  } catch {
    // Storage can be unavailable; the default is fine.
  }
  return DEFAULT_LOCALE;
}

export function saveDesktopLocale(locale: DesktopLocale): void {
  try {
    localStorage.setItem(LOCALE_KEY, locale);
  } catch {
    // Nothing to do: the preference just does not persist.
  }
}
