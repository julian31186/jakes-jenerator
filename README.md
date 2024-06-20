# jakes-jenerator
Generate a pdf resume using Jakes template from JSON input 


# CMD for Dockerfile
docker build -t jakes-jenerator . && docker run -v $(pwd)/src/output:/app/src/output --rm jakes-jenerator