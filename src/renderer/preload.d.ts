declare global {
  interface Window {
    electron: {
      ipcRenderer: {
        invoke(channel: string): Promise<unknown>;
        myPing(): void;
        on(
          channel: string,
          func: (...args: unknown[]) => void
        ): (() => void) | undefined;
        once(channel: string, func: (...args: unknown[]) => void): void;
        removeListener(channel: string, func: any): void;
      };
    };
  }
}

export {};
