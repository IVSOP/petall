ssh-keygen -t rsa -b 4096 -m PEM -f validation.key
openssl rsa -in validation.key -pubout -outform PEM -out validation.key.pub
