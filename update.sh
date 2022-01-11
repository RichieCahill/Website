#!/bin/bash
V=0.5.1
docker build -t themadmaker2/website:$V .
docker stop Website
docker rm Website
docker run -d --restart unless-stopped --name=Website -p 5000:5000 themadmaker2/website:$V
