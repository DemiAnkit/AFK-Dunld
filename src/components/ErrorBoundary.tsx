import React, { Component, ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Uncaught error:', error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="flex items-center justify-center h-screen bg-gray-950 text-white p-8">
          <div className="max-w-lg w-full bg-gray-900 rounded-lg border border-red-500/50 p-6">
            <div className="flex items-center gap-3 mb-4">
              <div className="p-2 bg-red-500/20 rounded-lg">
                <svg className="w-6 h-6 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
              </div>
              <h1 className="text-xl font-bold text-red-500">Application Error</h1>
            </div>
            
            <p className="text-gray-300 mb-4">
              Something went wrong while loading the application.
            </p>
            
            {this.state.error && (
              <div className="bg-gray-950 rounded p-4 mb-4 overflow-auto max-h-64">
                <p className="text-sm font-mono text-red-400">
                  {this.state.error.toString()}
                </p>
                {this.state.error.stack && (
                  <pre className="text-xs text-gray-500 mt-2 overflow-auto">
                    {this.state.error.stack}
                  </pre>
                )}
              </div>
            )}
            
            <button
              onClick={() => window.location.reload()}
              className="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
            >
              Reload Application
            </button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
