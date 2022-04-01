#IMAGE 
FROM arm64v8/registry:latest
COPY daemon.json /etc/docker/
EXPOSE 5000
CMD ["docker run -d -p 5000:5000 --restart always --name registry arm64v8/registry:2"]
