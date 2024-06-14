FROM node:alpine3.20

WORKDIR /app
COPY . .

RUN yarn install && yarn run build

EXPOSE 3000

CMD yarn run start
