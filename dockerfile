FROM nginx:stable-bookworm-perl
COPY nginx.conf /etc/nginx/nginx.conf
COPY ssl/* /ssl/*
EXPOSE 443