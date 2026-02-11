import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ErrorBoundary } from "./components/ErrorBoundary";
import "./styles/globals.css";

console.log("🚀 AFK-Dunld starting...");

// Wait for DOM to be ready
document.addEventListener('DOMContentLoaded', () => {
  console.log("✅ DOM ready");
  
  const rootElement = document.getElementById("root");
  
  if (!rootElement) {
    console.error("❌ Root element not found!");
    document.body.innerHTML = `
      <div style="display: flex; align-items: center; justify-content: center; height: 100vh; background: #0a0a0f; color: #ef4444; font-family: sans-serif; padding: 20px;">
        <div style="text-align: center;">
          <h1 style="font-size: 24px; margin-bottom: 10px;">Error: Root Element Not Found</h1>
          <p style="color: #9ca3af;">The application failed to initialize.</p>
        </div>
      </div>
    `;
    return;
  }

  console.log("✅ Root element found");

  // Add timeout for loading screen removal
  const loadingTimeout = setTimeout(() => {
    console.log("⏰ Loading timeout reached - forcing app render");
    const loadingScreen = rootElement.querySelector('.loading-screen');
    if (loadingScreen) {
      loadingScreen.remove();
    }
  }, 5000); // 5 second timeout

  try {
    console.log("🎨 Rendering React app...");
    
    // Create React root and render
    const root = ReactDOM.createRoot(rootElement);
    root.render(
      <React.StrictMode>
        <ErrorBoundary>
          <App />
        </ErrorBoundary>
      </React.StrictMode>
    );
    
    // Clear timeout and loading screen once React starts rendering
    clearTimeout(loadingTimeout);
    
    // Clear the loading screen after a short delay to allow React to mount
    setTimeout(() => {
      const loadingScreen = rootElement.querySelector('.loading-screen');
      if (loadingScreen) {
        loadingScreen.remove();
        console.log("✅ Loading screen cleared");
      }
    }, 100);
    
    console.log("✅ React app rendered successfully");
  } catch (error) {
    clearTimeout(loadingTimeout);
    console.error("❌ Failed to render React app:", error);
    rootElement.innerHTML = `
      <div style="display: flex; align-items: center; justify-content: center; height: 100vh; background: #0a0a0f; color: #ef4444; font-family: sans-serif; padding: 20px;">
        <div style="max-width: 600px;">
          <h1 style="font-size: 24px; margin-bottom: 10px;">Failed to Start Application</h1>
          <p style="color: #9ca3af; margin-bottom: 20px;">An error occurred while initializing the app.</p>
          <pre style="background: #1f2937; padding: 15px; border-radius: 8px; overflow: auto; font-size: 12px;">
${error instanceof Error ? error.stack : String(error)}
          </pre>
        </div>
      </div>
    `;
  }
});

// Log when window loads
window.addEventListener('load', () => {
  console.log("✅ Window fully loaded");
});

// Catch any unhandled errors
window.addEventListener('error', (event) => {
  console.error("❌ Unhandled error:", event.error);
});

// Catch any unhandled promise rejections
window.addEventListener('unhandledrejection', (event) => {
  console.error("❌ Unhandled promise rejection:", event.reason);
});
