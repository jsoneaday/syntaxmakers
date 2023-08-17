import { flushSync } from "react-dom";

export function startViewTransition(callback: () => void) {
  if (document.startViewTransition) {
    document.startViewTransition(() => {
      flushSync(() => callback());
    });
  }
}
