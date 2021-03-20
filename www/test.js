async function test() {
    console.log("Starting");
    const response = await fetch('/api/test', {
        method: 'POST',
        body: "{sample:'test'}\n"
    });
}