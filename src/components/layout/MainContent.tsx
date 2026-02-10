// src/components/layout/MainContent.tsx
import { Outlet } from "react-router-dom";

export function MainContent() {
  return (
    <main className="flex-1 overflow-auto bg-gray-950">
      <Outlet />
    </main>
  );
}
