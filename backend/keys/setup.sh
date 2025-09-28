ssh-keygen -t rsa -b 4096 -m PEM -f access.key
openssl rsa -in access.key -pubout -outform PEM -out access.key.pub
ssh-keygen -t rsa -b 4096 -m PEM -f refresh.key
openssl rsa -in refresh.key -pubout -outform PEM -out refresh.key.pub
