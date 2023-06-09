# stage 1 - generate a rust recipe file for dependencies
FROM rustlang/rust:nightly as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 2 - build our dependencies
FROM rustlang/rust:nightly as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json

# install cargo-leptos
RUN cargo install cargo-leptos

RUN cargo chef cook --release --recipe-path recipe.json

# stage 3 - use the main official docker image as builder
FROM rustlang/rust:nightly as builder

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get update && apt-get install nodejs

# copy the app in docker image
COPY . /app

# set up working directory
WORKDIR /app

# copy dependecies
COPY --from=cacher /app/target /target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# target wasm32-unknown-unknown 
RUN rustup target add wasm32-unknown-unknown 

# install sass
RUN npm install -g tailwindcss
RUN npx tailwindcss -i ./style/input.css -o ./style/main.css

# set env variables for build
# The browserlist https://browsersl.ist query used for optimizing the CSS.
ENV LEPTOS_BROWSERQUERY "defaults"

# build the app
RUN cargo leptos build --release

# use googles distroless as runtime image
FROM gcr.io/distroless/cc-debian11

# copy app form builder
COPY --from=builder /app/target /app
WORKDIR /app

# Site .env parameters cargo-leptos
ENV LEPTOS_OUTPUT_NAME "start-axum"
ENV LEPTOS_SITE_ROOT "site"
ENV LEPTOS_SITE_PKG_DIR "pkg"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT "3001"
EXPOSE "3000"

# start the application
CMD ["./server/release/start-axum"]
