# crustpass

## Description

Manage seed data for my home lab.

## Configuration

`SQ_CONFIGURATION_JSON_FILE` - Path to the settings file. Default: `app_settings.json`

`SQ_CONFIGURATION_JSON` - JSON string of settings. Default: `null`

Priorities: `SQ_CONFIGURATION_JSON_FILE` > `SQ_CONFIGURATION_JSON`

```json
{
    "server" : "See Server",
    "physical": "See Physical",
    "authentication": "See Authentication"
}
```

## Configuration: Server

Server Settings.

```json
{
    "socket_addr": "Listen address for the server, Example: `127.0.0.1:8080`",
}
```

## Configuration: Physical

Persistence layer for the seed data.

```json
{
    "physical_type": "Type of physical storage",
    "physical_details": "Details for the physical storage"
}
```

- `libsql`

    ```json
    {
        "db_url": "Database connection string",
        "auth_token": "Authentication token for the database",
        "table_name": "Table name for the seed data"
    }
    ```

## Configuration: Authentication

Authentication layer for the API.

```json
{
    "authentication_type": "Type of authentication",
    "authentication_details": "Details for the authentication"
}
```

- `admin_api_key`

    ```json
    {
        "api_key": "Admin API key"
    }
    ```
