# Secret Squirrel

## Description

Manage seed data for my home lab.

## Configuration

`SQ_SETTINGS_FILE` - Path to the settings file. Default: `app_settings.json`

`SQ_SETTINGS_JSON` - JSON string of settings. Default: `null`

Priorities: `SQ_SETTINGS_FILE` > `SQ_SETTINGS_JSON`

```json
{
    "socket_addr": "Listen address for the server, Example: `127.0.0.1:3000`",
    "physical": {
        "physical_type": "libsql", // Possible values: `libsql`
        "physical_details": null // Details for the physical type
    },
    "authentication": {
        "authentication_type": "Type of authentication", // Possible values: `admin_api_key`
        "authentication_details": null // Details for the authentication type
    }
}
```

## Configuration: Physical

### Configuration: Physical: libsql

```json
{
    "db_url": "Database connection string",
    "auth_token": "Authentication token for the database",
    "table_name": "Table name for the seed data"
}
```

## Configuration: Authentication

### Configuration: Authentication: admin_api_key

```json
{
    "api_key": "Admin API key"
}
```
