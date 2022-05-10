<p align="center">
  <img alt="Stump logo" src="./.github/images/logo.png" style="width: 50%" />
</p>

A free and open source comics server with OPDS support, **heavily** inspired by [Komga](https://github.com/gotson/komga), created with Rust, [Rocket](https://github.com/SergioBenitez/Rocket), [Prisma](https://github.com/Brendonovich/prisma-client-rust) and React.

I love Komga and use it at home, and I thought it would be cool to learn what goes into making something like this myself. I opted to develop this in Rust to hopefully, at the end of all this, create something just as if not almost as convenient but with a smaller footprint. _I also just want to practice Rust!_

## Roadmap

I'll list the major target features below - I am very open to suggestions and ideas, so feel free to reach out if you have anything you'd like to see!

- Full OPDS + OPDS Page Streaming support
- EPUB, PDF, and CBZ/CBR support
- Customizable configuration (for both Docker and local hosting)
- Scheduled and invokable filesystem indexing/scanning
- Support for a range of metadata operations (e.g. adding/removing tags, changing metadata, etc.)
- Import/export of libraries
- Configurable CBR-to-CBZ conversion
- Small footprint and resource utilization (Docker image size currently sits at ~41MB)
- Integrated web client (React) served by Rust server
  - Full Text Search
  - Server management
  - Built-in webreader for media
- Role-based access control (i.e. the server owner and authorized users)

## Project Structure

I am ommitting a lot of files and only focusing on the main directories, but the following is the structure of the project:

```bash
.
├── packages # workspaces root
│   ├── core # core package, stump lives here
│   │   ├── frontend # the built-in web client
│   │   └── server # stump core implementation
│   │       ├── prisma # prisma configuration
│   │       ├── prisma-cli # prisma CLI configuration
│   │       ├── src # source code
│   │           ├── bin # bin rust files
│   │               ├── seed.rs # seed database with fake data
│   └── website # the advertisement website code
├── README.md
└── ...
```

<!-- - `website` code is deployed to [stumpapp.dev](http://stumpapp.dev) -->

## Development Setup

Clone the repository and run the setup script:

```bash
git clone https://github.com/aaronleopold/stump.git
cd stump
pnpm run setup
```

This will install all the dependencies, generate the prisma client, and seed the database with some fake data.

## Running Stump

To start the application for development, simply run:

```bash
pnpm core dev
```

This will start both the vite dev server and the rust server, watching for changes. You can also run the server and the frontend in separate processes:

```bash
pnpm core server:dev # start the server
pnpm core frontend:dev # start the frontend
```

## Docker

<details>
<summary>
  <b>Note: This is currently non-functional. Migrating to Prisma from SeaORM bork this, but I am working on it.</b>
</summary>

No images have been published to dockerhub yet, so you'll have to build it yourself:

```bash
 # build the docker image
pnpm core build:docker
# create the docker container
docker create \
  --name=stump \
  --user 1000:1000 \
  -p 6969:6969 \
  --volume ~/.stump:/home/stump/.stump \
  --mount type=bind,source=/path/to/data,target=/data \
  --restart unless-stopped \
  stump
# run the docker container
docker start stump
```

As of now, you'll need to make the `source` and `target` paths match. So if you keep your libraries in `/Users/user/Library`, you'll need to bind `/Users/user/Library` to both `source` and `target`. This will eventually change to be more flexible.

</details>

## Contributing

Contributions are very **encouraged** and **welcome**! Please review the [CONTRIBUTING.md](./CONTRIBUTING.md) file beforehand. Thanks!
