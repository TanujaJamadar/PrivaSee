// scanner.js
const puppeteer = require("puppeteer");

const targetUrl = process.argv[2]; // Rust sends URL as 1st argument

if (!targetUrl) {
  console.error(JSON.stringify({ type: "error", message: "No URL provided" }));
  process.exit(1);
}

(async () => {
  try {
    // Send status to Rust
    console.log(
      JSON.stringify({ type: "status", message: "Launching Puppeteer..." }),
    );

    const browser = await puppeteer.launch({
      headless: "new", // The setting that worked for you!
      args: ["--no-sandbox", "--disable-setuid-sandbox"],
    });

    const page = await browser.newPage();

    // Basic Stealth
    await page.setUserAgent(
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    );

    // Intercept Requests
    await page.setRequestInterception(true);

    page.on("request", (req) => {
      const url = req.url();

      // Filter out data: URIs to keep logs clean
      if (!url.startsWith("data:")) {
        // Send RAW traffic data to Rust
        console.log(
          JSON.stringify({
            type: "traffic",
            data: {
              url: url,
              method: req.method(),
              resourceType: req.resourceType(),
              postData: req.postData(), // Capture POST body for analysis
            },
          }),
        );
      }
      req.continue();
    });

    console.log(
      JSON.stringify({
        type: "status",
        message: `Navigating to ${targetUrl}...`,
      }),
    );

    // Go to page
    await page.goto(targetUrl, {
      waitUntil: "domcontentloaded",
      timeout: 30000,
    });

    console.log(
      JSON.stringify({ type: "status", message: "Page Loaded. Scrolling..." }),
    );

    // Scroll to trigger lazy trackers (TikTok/FB)
    await page.evaluate(async () => {
      await new Promise((resolve) => {
        let totalHeight = 0;
        const distance = 100;
        const timer = setInterval(() => {
          const scrollHeight = document.body.scrollHeight;
          window.scrollBy(0, distance);
          totalHeight += distance;
          if (totalHeight >= scrollHeight) {
            clearInterval(timer);
            resolve();
          }
        }, 100);
      });
    });

    // Wait a bit for final pixels to fire
    await new Promise((r) => setTimeout(r, 3000));

    console.log(JSON.stringify({ type: "status", message: "Scan Complete." }));
    await browser.close();
  } catch (error) {
    console.log(JSON.stringify({ type: "error", message: error.message }));
    process.exit(1);
  }
})();
