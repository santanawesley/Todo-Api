# TODOS API

This is a sample REST api made in Rust

## Local deployment
The recomended way to use this api in local development is using docker, there is a docker-compose  file available
```
docker-compose up -d
```

## Usage

All the CRUD operations are available in the endpoint **/api/todos**.  

### Creating a new TODO

When creating a TODO, you can specify the **name** only.

```
curl -d '{"name":"new todo"}' -H "Content-Type: application/json" -X POST http://localhost:7000/api/todos
```

### Updating an existing TODO

When updating a TODO, you can specify the **name** or **completed** fields
```
curl -d '{ "name":"new todo", "completed": true}' -H "Content-Type: application/json" -X PUT http://localhost:7000/api/todos/1
```

### Deleting an existing TODO
```
curl -X DELETE http://localhost:7000/api/todos/1
```

### Listing TODOs
```
curl -X GET http://localhost:7000/api/todos
```


### Returning a TODO by Id
```
curl -X GET http://localhost:7000/api/todos/1
```
