const CACHE_NAME = 'game-of-life-v1_0_1';

// 安裝 Service Worker
self.addEventListener('install', (event) => {
  self.skipWaiting();
});

// 當新的 SW 啟動並接管時，清除舊快取並立即控制頁面
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cache) => {
          return caches.delete(cache);
        })
      );
    }).then(() => self.clients.claim())
  );
});

// 監聽來自網頁端的訊息 (接收 SKIP_WAITING 指令)
self.addEventListener('message', (event) => {
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }
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