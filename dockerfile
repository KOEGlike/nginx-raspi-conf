FROM nginx:stable-bookworm-perl
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 443