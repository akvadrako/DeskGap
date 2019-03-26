import { entryPath, NativeException } from './internal/bootstrap';
import app from './app';
import { BrowserWindow } from './browser-window';
import { Menu, MenuItem } from './menu';
import { WebViews, messageNode } from './webview';
import Dialog from './dialog';
import shell from './shell';
import './async-node';
import systemPreferences from './system-preferences';
import os = require('os');

export = {
    app,
    BrowserWindow,
    webViews: WebViews,
    webContents: WebViews,
    Menu,
    MenuItem,
    messageNode,
    ipcMain: messageNode,
    systemPreferences,
    dialog: Dialog,
    NativeException,
    shell,
};


process.on('uncaughtException', (error) => {
    if (process.listeners('uncaughtException').length > 1) {
        return;
    }
    const message = error.stack || `${error.name}: ${error.message}`;
    Dialog.showErrorBox('Uncaught exception', message);
    process.exit(1);
});

process.nextTick(() => {
    app['run_']();
    __non_webpack_require__(entryPath);
});
