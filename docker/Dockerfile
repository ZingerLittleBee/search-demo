FROM node:lts-alpine3.19 as frontend

WORKDIR /usr/src/muse

COPY ./ui .

RUN npm install -g pnpm

RUN pnpm i

RUN pnpm build


FROM rust:alpine3.19 as builder

WORKDIR /usr/src/muse

COPY . .

COPY --from=frontend /usr/src/muse/dist /usr/src/muse/ui/dist

# https://github.com/hound-search/hound/issues/238
RUN apk add --no-cache cmake musl-dev build-base libressl-dev

RUN SKIP_BUILD_SCRIPT=1 cargo build --release


FROM alpine:3.19

COPY --from=builder /usr/src/muse/target/release/search-demo /usr/local/bin/search-demo

EXPOSE 3000

ENTRYPOINT [ "search-demo" ]
