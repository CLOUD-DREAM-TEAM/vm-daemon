## Environment Variables

| Variable | Description | Required |
|---|---|---|
| `RUST_LOG` | Log level (`error`, `info`, `debug`, `trace`) | No (default: `debug`) |
| `ORCHESTRATOR_URL` | URL of the orchestrator service | Yes |
| `LOGS_PORT` | Port for the logs endpoint | Yes |
| `VM_REPORT_INTERVAL` | Metric reporting interval in seconds | Yes |
| `VM_ID` | Identifier for this VM instance | Yes |

## Local Development

### Run with Docker Compose

Set environment variables in `docker-compose.yml` as appropriate, then:

```sh
docker compose up -d --build --force-recreate
```

## Cloud Deployment

`cloud-init.sh` is a cloud-init script that installs Docker and runs a version of this repository hosted on `ghcr.io/CLOUD-DREAM-TEAM/vm-runner:latest`

The following environment variables must be set on the VM before `cloud-init.sh` runs:

- `ORCHESTRATOR_URL`
- `LOGS_PORT`
- `VM_REPORT_INTERVAL`
- `VM_ID`
