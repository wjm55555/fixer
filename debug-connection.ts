
const apiKey = "sk-veCdMcFWqwQVIcSpltf5RpkcSWSLk0Ivk3z3JxirOUuDgVSc";
const baseUrl = "https://anyrouter.top/v1/chat/completions";

async function testConnection() {
    console.log("Testing connection to:", baseUrl);
    try {
        const response = await fetch(baseUrl, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Bearer ${apiKey}`,
                "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
            },
            body: JSON.stringify({
                model: "claude-haiku-4-5-20251001",
                messages: [{ role: "user", content: "Hello" }],
                stream: false
            })
        });

        console.log("Status:", response.status);
        console.log("Headers:", JSON.stringify([...response.headers.entries()]));
        const text = await response.text();
        console.log("Body length:", text.length);
        console.log("Body preview:", text.slice(0, 500));
    } catch (error) {
        console.error("Connection failed:", error);
    }
}

testConnection();
