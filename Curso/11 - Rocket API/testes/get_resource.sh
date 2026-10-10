# Sem token de autorização
curl localhost:8000/resources/1

# Com token de autorização
curl -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIiwiZXhwIjoxNzgzMDUxMDk0fQ.qdt35cuiz1Yy3k6czAGEif9hrfBwJMHVt0WEOWKb-lU" localhost:8000/resources/1
