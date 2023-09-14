const { invoke } = window.__TAURI__.tauri;

function setupRouteHandler(routeId) {
    let greetingMessageElement;
    let routeData;
    const selector = `#${routeId}`;
    const routeElement = document.querySelector(selector);
    
    async function handleRouteData() {
        greetingMessageElement.textContent = await invoke("route_system", { data: routeData });
    }
    
    window.addEventListener("DOMContentLoaded", () => {
        routeData = routeElement.getAttribute('data-url');
        greetingMessageElement = document.querySelector("#greet-msg");
        
        routeElement.addEventListener("click", (e) => {
            e.preventDefault();
            handleRouteData();
        });
    });
}

setupRouteHandler('route_1');
setupRouteHandler('route_2');
