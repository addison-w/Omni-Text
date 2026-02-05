import { getCurrentWindow } from "@tauri-apps/api/window";

let theme = $state<"light" | "dark">("dark");

async function initTheme() {
  try {
    const appWindow = getCurrentWindow();
    const currentTheme = await appWindow.theme();
    theme = currentTheme === "light" ? "light" : "dark";

    await appWindow.onThemeChanged(({ payload }) => {
      theme = payload === "light" ? "light" : "dark";
    });
  } catch (e) {
    // Default to dark if detection fails
    console.error("Theme detection failed:", e);
  }
}

export const themeStore = {
  get theme() { return theme; },
  initTheme,
};
