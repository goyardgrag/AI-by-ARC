

<img src="https://i.imgur.com/q0BMRZU.png" style="max-width: 100%; height: auto;">
<h1>ARCAI </h1>
<p><span style="text-transform: uppercase;">An AI agent powered by <a href="https://github.com/0xPlaygrounds/rig/tree/main" target="_blank"><strong>rig</strong></a> for enhanced functionality. Share your ideas and suggestions to help improve ARCAI.</span></p>

<p><span style="text-transform: uppercase;">A fully functional version of ARCAI is currently available. You can learn more about how it works on <a href="https://x.com/arcai_project" target="_blank"><strong>Twitter</strong></a> 
<h1>ARCAI roadmap </h1>

<h2 style="text-transform: uppercase;">Phase 1: Initial Development</h2>
<ul>
  <li>🟢 Develop the AI bot using the RIG framework</li>
  <li>🟢 Integrate the bot into Twitter</li>
  <li>🟢 Launch on Pumpfun</li>
  <li>🟢 Execute the first phase of marketing with the goal of trending across all platforms</li>
</ul>

<h2 style="text-transform: uppercase;">Phase 2: Optimization and Expansion</h2>
<ul>
  <li>🟣 Optimize the AI agent with a strong focus on improving its performance and quality</li>
  <li>🟣 Integrate $ARCAI into Telegram</li>
  <li>🟣 Second phase of marketing, focusing on KOL partnerships and collaborations</li>
</ul>

<h2 style="text-transform: uppercase;">Phase 3: Full Interaction and Advanced Features</h2>
<ul>
  <li>⚫️ Enable full user interaction with ARCAI in Telegram for everyone</li>
  <li>⚫️ Add advanced features, including image generation</li>
  <li>⚫️ Third phase of marketing, targeting key listings on Bybit, CoinGecko, CoinMarketCap</li>
</ul>


<h1>AI by ARC - Installation and Setup Guide</h1>

<p><strong><span style="text-transform: uppercase;">Prerequisites</span></strong></p>

<p>To install and run the project, you will need:</p>

<ul>
  <li><span style="text-transform: uppercase;">Rust</span> — the latest stable version. You can install it <a href="https://www.rust-lang.org/tools/install" target="_blank"><strong>here</strong></a>.</li>
  <li><span style="text-transform: uppercase;">API Keys and Credentials</span>:
    <ul>
      <li><span style="text-transform: uppercase;">Twitter API v2 (OAuth 1.0a)</span>: For Twitter integration.</li>
      <li><span style="text-transform: uppercase;">Other keys</span> (if applicable): Depending on the project's configuration.</li>
    </ul>
  </li>
</ul>


<p><strong><span style="text-transform: uppercase;">Installation</span></strong></p>

<p>Clone the repository:</p>
<pre>
<code>git clone https://github.com/gregoryaidev/AI-by-ARC.git
cd AI-by-ARC
</code>
</pre>

<p>Install dependencies:</p>
<p>Rust uses Cargo to manage dependencies. All required dependencies will automatically be installed from <strong>Cargo.toml</strong> during the build process.</p>

<p>Configure the environment file:</p>
<p>Create a <code>.env</code> file in the root directory of the project and add the required keys:</p>
<pre>
<code>TWITTER_API_KEY=your_twitter_api_key
TWITTER_API_SECRET=your_twitter_api_secret
TWITTER_ACCESS_TOKEN=your_twitter_access_token
TWITTER_ACCESS_SECRET=your_twitter_access_secret
CHARACTER_NAME=your_character_name
</code>
</pre>



<p><strong><span style="text-transform: uppercase;">Setting Up Your Character</span></strong></p>

<p>The project allows you to configure an AI agent with unique characteristics.</p>

<p>Create a folder for the character:</p>
<pre>
<code>mkdir characters/{CHARACTER_NAME}
</code>
</pre>

<p>Set up the character file:</p>
<p>Inside the character folder, create a file named <code>character.json</code> with the following structure:</p>
<pre>
<code>{
    "instructions": {
        "base": "Base instructions for the bot",
        "suffix": "Additional instructions"
    },
    "adjectives": ["smart", "interactive"],
    "bio": {
        "headline": "Character description",
        "key_traits": ["trait1", "trait2"]
    },
    "lore": ["background1", "background2"],
    "styles": ["style1", "style2"],
    "topics": ["topic1", "topic2"],
    "post_style_examples": ["example1", "example2"]
}
</code>
</pre>

<p><strong><span style="text-transform: uppercase;">Running the Project</span></strong></p>

<p>Run the project:</p>
<pre>
<code>cargo run
</code>
</pre>
<p>This will start the main program and execute its functions based on the provided configurations.</p>

<p><strong>Testing:</strong></p>
<p>Verify that the bot interacts with Twitter correctly and performs as expected.</p>

<p><strong><span style="text-transform: uppercase;">Project Structure</span></strong></p>
<pre>
<code>AI-by-ARC/
├── Cargo.toml       # Project configuration and dependencies
├── README.md        # Project documentation
├── main.rs          # Main program code
└── characters/      # Character configurations
</code>
</pre>

<p><strong><span style="text-transform: uppercase;">Dependencies</span></strong></p>
<ul>
  <li><strong>tokio</strong> — Asynchronous library for task processing.</li>
  <li><strong>serde</strong> — Library for serialization and deserialization of data.</li>
  <li><strong>Twitter API</strong> — For interaction with Twitter.</li>
  <li><strong>RIG</strong> — Framework for building modular AI agents.</li>
</ul>

<p><strong><span style="text-transform: uppercase;">Acknowledgments</span></strong></p>
<ul>
  <li>The <strong>RIG</strong> team for providing the framework for AI agents.</li>
  <li>All developers and contributors to the project.</li>
</ul>

