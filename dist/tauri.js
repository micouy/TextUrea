window.onload = onload;

function onload() {
    const { window: tauriWindow, globalShortcut, clipboard } = window.__TAURI__;

    const mainWindow = tauriWindow.appWindow;
    
    const textarea = u("#editor-input").first();

    mainWindow.listen('copy', async ({ event, payload }) => {
        await clipboard.writeText(textarea.value);
    });

    globalShortcut.unregister('CommandOrControl+Shift+`');
    globalShortcut.register('CommandOrControl+Shift+`', async () => {
        let text = await clipboard.readText();
        textarea.value = text;

        await mainWindow.show();
        await mainWindow.setFocus();

        return Promise().resolve(true);
    })
}
