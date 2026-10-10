# Sem token de autorização
curl -X POST -H "Content-Type: application/json" -d '{"title": "Recurso 3", "description": "O recurso 3"}' localhost:8000/resources

# Com token de autorização
curl -X POST -H "Content-Type: application/json" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIiwiZXhwIjoxNzgzMDUxMDk0fQ.qdt35cuiz1Yy3k6czAGEif9hrfBwJMHVt0WEOWKb-lU" -d '{"title": "Recurso 3", "description": "O recurso 3"}' http://localhost:8000/resources
