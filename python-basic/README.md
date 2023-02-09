# python-basic project template

Python with some minimal tooling.

## Requirements

- Nix

## Getting started

```bash
# Choose a name for your project
export PROJECT="my-project"
# Create project from Nix Template and move into its directory
nix flake new -t github:panicmacc/start#python-basic ${PROJECT}
cd ${PROJECT}
# Activate Nix development environment
nix develop
# Create a Python venv and activate it
python -m venv ./venv
. ./venv/bin/activate
# Install any needed python packages with pip
pip install <...>
# Develop and run python code
python <...>
```

When done, deactivate the python venv and exit the Nix environment.

```bash
deactivate
exit
```

