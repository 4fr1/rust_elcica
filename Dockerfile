FROM nginx:alpine

COPY templates/index.html /tmp/index.template.html

RUN sed 's/{{ title }}/ELCICA/g' /tmp/index.template.html \
    > /usr/share/nginx/html/index.html

EXPOSE 80
