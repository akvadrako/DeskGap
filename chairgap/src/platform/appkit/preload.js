window.__deskgap = {
    platform: 'darwin',
    postStringMessage: function (string) {
         window.webkit.messageHandlers.stringMessage.postMessage(string);
    },
    startDragging: function (string) {
        window.webkit.messageHandlers.windowDrag.postMessage(null);
    }
};
