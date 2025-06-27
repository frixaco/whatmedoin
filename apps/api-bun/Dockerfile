FROM oven/bun:1 as builder

WORKDIR /app

COPY package.json  ./

RUN bun install --frozen-lockfile

COPY . .

RUN bun run build

FROM oven/bun:1-slim as runner

WORKDIR /app

COPY --from=builder /app/server.js ./

ENV NODE_ENV=production
ENV PORT=3000

EXPOSE 3000

USER bun

CMD ["bun", "run", "./server.js"]

