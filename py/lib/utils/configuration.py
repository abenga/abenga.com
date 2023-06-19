import toml
import os


def get_config():
    directory_path = os.path.dirname(__file__)
    config_path = os.path.join(directory_path, "..", "..", "..", "configuration.toml")
    with open(config_path) as f:
        parsed_config = toml.load(f)
        return parsed_config
