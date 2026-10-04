    for name in ["visibilitychange", "freeze", "resume"] { listeners.listen(document.as_ref(), name)?; }
    for name in ["error", "unhandledrejection"] { listeners.listen(window.as_ref(), name)?; }