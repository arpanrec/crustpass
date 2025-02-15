# crustpass

## Description

Manage seed data for my home lab.

## Configuration

`SQ_SETTINGS_JSON_FILE` - Path to the settings file. Default: `app_settings.json`

`SQ_SETTINGS_JSON` - JSON string of settings. Default: `null`

Priorities: `SQ_SETTINGS_JSON_FILE` > `SQ_SETTINGS_JSON`

```json
{
    "socket_addr": "Listen address for the server, Example: `127.0.0.1:3000`",
    "physical": {
        "physical_type": "See Physical",
        "physical_details": "Details for the physical storage"
    },
    "authentication": {
        "authentication_type": "See Authentication",
        "authentication_details": "Details for the authentication"
    }
}
```

## Configuration: Physical

Persistence layer for the seed data.

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

- `admin_api_key`

    ```json
    {
        "api_key": "Admin API key"
    }
    ```
