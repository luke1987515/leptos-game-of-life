const CACHE_NAME = 'game-of-life-v1';

self.addEventListener('install', (event) => {
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  event.waitUntil(clients.claim());
});

self.addEventListener('fetch', (event) => {
  const request = event.request;
  const url = new URL(request.url);

  // 關鍵過濾：只快取 HTTP / HTTPS 請求，忽略 chrome-extension:// 等其他 Scheme
  if (url.protocol !== 'http:' && url.protocol !== 'https:') {
    return;
  }

  event.respondWith(
    caches.match(request).then((response) => {
      if (response) {
        return response; // 命中快取，離線直接回傳
      }
      return fetch(request).then((fetchResponse) => {
        if (!fetchResponse || fetchResponse.status !== 200 || fetchResponse.type !== 'basic' || request.method !== 'GET') {
          return fetchResponse;
        }
        const responseToCache = fetchResponse.clone();
        caches.open(CACHE_NAME).then((cache) => {
          cache.put(request, responseToCache);
        });
        return fetchResponse;
      });
    })
  );
});