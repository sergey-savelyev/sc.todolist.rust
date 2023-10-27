FROM node:18

WORKDIR /usr/src/app

COPY package*.json ./
RUN npm install

COPY . .

RUN npm run build

EXPOSE 4173
ENV HOST=0.0.0.0

CMD [ "npm", "run", "preview", "--", "--host", "0.0.0.0" ]