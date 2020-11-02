# rs-connect

Generally I have faced problems for keeping dev, prod credentials(database, remote machine) in a text file and then 
copy command/password into terminal and connect. Then to figure out how much time I have expanded there on remote, 
I have to create too many aliases and keep in a bash file and wrap every alias with time start and end.

In this project, I would automate this work using rust. Mostly when we are running command, we have to pass env, 
command and arguments. 

For example If we have to connect with postgre sql

```
PGPASSWORD="root" psql -h localhost -U root -d temp
``` 

## Now check it out

For this, Firstly we have to create a json file with the following format and set the path in the environment variable.
```export CONNECT_CONFIG=<json file path>```

```json
{
  "prod_machine": {
    "command": "ssh",
    "args": ["ubuntu@192.168.0.1"],
    "envs": []
  },
  "dev_db": {
    "command": "psql",
    "args": ["-h", "192.168.0.1", "-U", "root", "-d", "temp"],
    "envs": {
      "PGPASSWORD": "root"
    }
  }
}
```

Now build binaries using `cargo build --release` and create an alias for running.
```
alias connect=./target/release/rs-connect
```

Now run command which you make keys in json at root level.

```
connect prod_machine
```

```
connect dev_db
```

For this library is supporting single command line arguments, command that you want to run.

## TODO
- spent time
- export command logs and time
- `connect which prod*`, It will describe all command which have prefix with `prod`
- args --print-args, --print-envs, --print-command, --print, -h