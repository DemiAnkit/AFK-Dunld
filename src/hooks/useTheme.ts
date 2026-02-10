import { useEffect } from "react";
import { useSettingsStore } from "../stores/settingsStore";

export function useTheme() {
  const { settings } = useSettingsStore();

  useEffect(() => {
    const theme = settings?.theme || "dark";
    document.documentElement.classList.remove("light", "dark");
    document.documentElement.classList.add(theme);
  }, [settings?.theme]);
}
