# Website

This website is built using [Docusaurus](https://docusaurus.io/), a modern static website generator.

### Installation

```bash
npm install
cd slides
npm install
```

### Local Development

#### Website

```bash
npm start
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

#### Slides

```bash
cd slides
npm run dev -- lectures/$lecture/slides.md
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

### Build

#### Build Website

```bash
npm run build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

#### Build Slides

```bash
cd slides
./build.sh
```

This command generates static content into the `build/slides` directory and can be served using any static contents hosting service.
