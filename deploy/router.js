// Route all requests to the getsundry.app domain via a Cloudflare worker,
// since we have multiple fly services as well as webflow landing pages.
// https://dash.cloudflare.com/7bb28e59c0f0430711e231bb00fd2fbd/workers/services/edit/sundry-router/production

const WEBFLOW_DOMAIN = 'www.getsundry.app'
const DOCS_PATH_PREFIX = '/reference/'
const API_PATH_PREFIX = '/v1/'
const WEBSOCKET_TIMEOUT = 30000
const README_DOCS_URL = 'https://sundry.readme.io'

addEventListener('fetch', event => {
    event.respondWith(handleRequest(event.request))
})

// Determine which service to use based on the request path
function getServiceFromPath(pathname) {
    if (pathname.startsWith(DOCS_PATH_PREFIX)) {
        return 'docs'
    } else if (pathname.startsWith(API_PATH_PREFIX)) {
        return 'sundry-api'
    } else {
        return 'sundry-website'
    }
}

async function handleRequest(request) {
    const url = new URL(request.url)

    if (url.hostname === WEBFLOW_DOMAIN) {
        // Log detailed information for DNS misconfiguration debugging
        console.error({
            error: 'DNS Misconfiguration detected',
            hostname: url.hostname,
            pathname: url.pathname,
            userAgent: request.headers.get('User-Agent'),
            referer: request.headers.get('Referer'),
            clientIP: request.headers.get('CF-Connecting-IP')
        })

        return new Response(JSON.stringify({ error: 'Not Found' }), {
            status: 404,
            headers: { 'Content-Type': 'application/json' }
        })
    }

    // Redirect root path to Webflow domain
    if (url.pathname === '/' || url.pathname === '') {
        return Response.redirect(`https://${WEBFLOW_DOMAIN}${url.search}`, 301)
    }

    // Log critical request information for debugging and monitoring
    console.log({
        hostname: url.hostname,
        pathname: url.pathname,
        fullUrl: request.url,
        upgrade: request.headers.get('Upgrade')
    })

    // Handle WebSocket upgrade requests before regular HTTP traffic
    const upgrade = request.headers.get('Upgrade')
    if (upgrade?.toLowerCase() === 'websocket') {
        return proxyWebSocketToFly(getServiceFromPath(url.pathname), request)
    }

    const service = getServiceFromPath(url.pathname)

    // Handle readme.io docs proxy
    if (service === 'docs') {
        return proxyToDocs(request)
    }

    return proxyToFly(service, request)
}

// Create a Fly.io request configuration with proper headers and URL
function createFlyRequest(service, request, options = {}) {
    const url = new URL(request.url)
    const flyHost = `${service}.fly.dev`

    const headers = new Headers(request.headers)
    headers.set('Host', flyHost)

    return {
        url: `https://${flyHost}${url.pathname}${url.search}`,
        config: {
            headers,
            method: request.method,
            body: request.body,
            ...options
        }
    }
}

async function proxyWebSocketToFly(service, request) {
    const { url, config } = createFlyRequest(service, request, {
        upgrade: ['websocket'],
        webSocket: {
            connectTimeout: WEBSOCKET_TIMEOUT,
            subprotocols: request.headers.get('Sec-WebSocket-Protocol')?.split(',').map(p => p.trim()) || []
        }
    })

    // Cloudflare requires explicit WebSocket upgrade configuration
    return fetch(url, config)
}

async function proxyToFly(service, request) {
    const { url, config } = createFlyRequest(service, request)
    const response = await fetch(url, config)
    const responseHeaders = new Headers(response.headers)

    return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers: responseHeaders
    })
}

async function proxyToDocs(request) {
    const url = new URL(request.url)
    const readmeUrl = new URL(README_DOCS_URL)
    const targetUrl = `${README_DOCS_URL}${url.pathname}${url.search}`

    const headers = new Headers(request.headers)
    headers.set('Host', readmeUrl.host)

    const response = await fetch(targetUrl, {
        method: request.method,
        headers,
        body: request.body
    })

    const responseHeaders = new Headers(response.headers)

    return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers: responseHeaders
    })
}
