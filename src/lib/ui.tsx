/** Minimal UI primitives for the overlay and pin windows.
 *
 * Only two things were needed from a shared kit: a theme provider and an icon
 * button. Both come straight from Mantine, so this app carries no private
 * dependency.
 */
import { ActionIcon, MantineProvider, createTheme } from '@mantine/core';
import type {
  ButtonHTMLAttributes,
  ComponentPropsWithoutRef,
  ReactElement,
  ReactNode,
} from 'react';
import '@mantine/core/styles.css';

const theme = createTheme({
  primaryColor: 'gray',
  fontFamily: 'system-ui, -apple-system, "Segoe UI", "PingFang SC", sans-serif',
  defaultRadius: 'md',
});

export function ShotmarkUiProvider({ children }: { children: ReactNode }): ReactElement {
  return (
    <MantineProvider theme={theme} forceColorScheme="dark">
      {children}
    </MantineProvider>
  );
}

export type IconButtonProps = Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'aria-label'> & {
  label: string;
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | number;
};

export function IconButton({
  label,
  className,
  title,
  type = 'button',
  color: _nativeColor,
  size = 'md',
  ...buttonProps
}: IconButtonProps): ReactElement {
  const actionIconProps = buttonProps as ComponentPropsWithoutRef<typeof ActionIcon>;
  return (
    <ActionIcon
      {...actionIconProps}
      type={type}
      className={className ? `shotmark-icon-button ${className}` : 'shotmark-icon-button'}
      aria-label={label}
      title={title ?? label}
      variant="subtle"
      size={size}
    />
  );
}
