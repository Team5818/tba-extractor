from argparse import Action
from typing import Type, Union


class Option:
    def __init__(self,
                 flag: str,
                 help: str,
                 required: bool = True,
                 action: Union[str, Type[Action]] = 'store'):
        self.flag = flag
        self.help = help
        self.required = required
        self.action = action


class Switch(Option):
    def __init__(self, flag: str, help: str):
        super().__init__(flag, help, required=False, action='store_true')
