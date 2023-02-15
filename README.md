# Install node version manager
```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
```
# Install and use node version
```
nvm install 18.14.0
nvm use 18.14.0
```
# Install node modules for the app
```
npm install
```
# Start and seed DB
```
docker-compose up
```
# Build and start spin app
```
spin build && spin up --follow-all --listen localhost:3000
```