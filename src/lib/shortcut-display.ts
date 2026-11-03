/** Keycap rendering for shortcut chords, adapted per platform. */

export type ShortcutDisplayPlatform = 'apple' | 'pc';

const APPLE_MODIFIER_GLYPHS = new Set(['⌘', '⌃', '⌥', '⇧']);

export function splitShortcutChord(chord: string): string[] {
  const tokens: string[] = [];
  let index = 0;
  while (index < chord.length) {
    const character = chord[index];
    if (character !== undefined && APPLE_MODIFIER_GLYPHS.has(character)) {
      tokens.push(character);
      index += 1;
      continue;
    }
    // The remaining substring is the key (may be multi-character, e.g. "F5").
    tokens.push(chord.slice(index));
    break;
  }
  return tokens;
}

/** Apple glyphs on macOS; Ctrl/Alt/Shift wording elsewhere. */
export function detectShortcutDisplayPlatform(
  userAgent: string = typeof navigator === 'undefined' ? '' : navigator.userAgent,
  platform: string = typeof navigator === 'undefined' ? '' : navigator.platform,
): ShortcutDisplayPlatform {
  if (/Mac|iPhone|iPad|iPod/i.test(platform) || /Mac OS|Macintosh|iPhone|iPad/i.test(userAgent)) {
    return 'apple';
  }
  return 'pc';
}

function remapTokenForPlatform(token: string, displayPlatform: ShortcutDisplayPlatform): string {
  if (displayPlatform === 'apple') {
    return token;
  }
  if (token === '⌘' || token === '⌃') {
    return 'Ctrl';
  }
  if (token === '⌥') {
    return 'Alt';
  }
  if (token === '⇧') {
    return 'Shift';
  }
  return token;
}

/** Tokens ready for keycap rendering on the current platform. */
export function formatShortcutChordTokens(
  chord: string,
  displayPlatform: ShortcutDisplayPlatform = detectShortcutDisplayPlatform(),
): string[] {
  return splitShortcutChord(chord).map((token) => remapTokenForPlatform(token, displayPlatform));
}
